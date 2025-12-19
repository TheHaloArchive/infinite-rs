//! Main abstraction file for modules.

use byteorder::{LE, ReadBytesExt};
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    path::Path,
    ptr::eq,
};

use super::{
    block::ModuleBlockEntry,
    file::{DataOffsetType, ModuleFileEntry},
    header::{ModuleHeader, ModuleVersion},
};
use crate::Result;
use crate::{
    Error,
    common::{errors::TagError, extensions::BufReaderExt},
};

#[derive(Default, Debug)]
/// Module structure which contains the layout of the entire module file.
pub struct ModuleFile {
    /// Information relating to how the other fields should be read.
    pub header: ModuleHeader,
    /// Metadata regarding compression and layout of files (tags).
    pub files: Vec<ModuleFileEntry>,
    /// Indices of resource files present in the module.
    pub resource_indices: Vec<u32>,
    /// Uncompressed/compressed blocks making up a file.
    blocks: Vec<ModuleBlockEntry>,
    /// Offset in [`BufReader`] where file data starts.
    file_data_offset: u64,
    /// Reference to the module file buffer.
    file_handle: Option<BufReader<File>>,
    /// Reference to HD1 buffer if it exists.
    hd1_file: Option<BufReader<File>>,
    /// Whether to use the HD1 module or not.
    pub use_hd1: bool,
}

impl ModuleFile {
    /// Instantiates a [`ModuleFile`] object from the given file path.
    pub fn from_path<T: AsRef<Path>>(file_path: T) -> Result<Self> {
        let mut module = Self::default();
        module.read(file_path)?;
        Ok(module)
    }

    /// Reads the module file from the given file path.
    /// This function reads the entire structure of the module file.
    /// It also calculates and stores important offsets within the file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to a type that implements [`Path`] that holds the path to the module file.
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    /// - If the string table has invalid UTF-8 [`Utf8ReadingError`](`crate::Error::Utf8ReadingError`)
    pub fn read<T: AsRef<Path>>(&mut self, file_path: T) -> Result<()> {
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);

        self.header.read(&mut reader)?;
        self.open_hd1(file_path)?;

        for _ in 0..self.header.file_count {
            let mut file = ModuleFileEntry::default();
            file.read(&mut reader, self.header.version == ModuleVersion::Flight1)?;
            self.files.push(file);
        }

        let strings_offset = reader.stream_position()?;
        reader.seek(SeekFrom::Start(
            strings_offset + u64::from(self.header.strings_size),
        ))?;
        self.resource_indices = (0..self.header.resource_count)
            .map(|_| -> Result<u32> { Ok(reader.read_u32::<LE>()?) })
            .collect::<Result<Vec<_>>>()?;
        let post_resource_offset = reader.stream_position()?;

        // Read strings contained in the file. A stringlist only exists in files before Season 3.
        // Each entry is separated by a null terminator, and files specify their offset themselves

        // in no particular order, so we cannot pre-read and just index into them.
        //
        // For files from modules that do not contain strings, we get it from the `get_tag_path` function.
        reader.seek(SeekFrom::Start(strings_offset))?;
        if self.header.version <= ModuleVersion::CampaignFlight {
            for file in &mut self.files {
                reader.seek(SeekFrom::Start(
                    strings_offset + u64::from(file.name_offset),
                ))?;
                file.tag_name = reader.read_null_terminated_string()?;
            }
        } else {
            let tag_paths: Vec<String> = (0..self.files.len())
                .map(|i| self.get_tag_path(i, 0))
                .collect::<Result<Vec<_>>>()?;

            for (file, tag_path) in self.files.iter_mut().zip(tag_paths) {
                file.tag_name = tag_path;
            }
        }

        reader.seek(SeekFrom::Start(post_resource_offset))?;
        self.blocks =
            reader.read_enumerable::<ModuleBlockEntry>(u64::from(self.header.block_count))?;

        // Align to 0x?????000
        let stream_position = reader.stream_position()?;
        reader.seek(SeekFrom::Start((stream_position / 0x1000 + 1) * 0x1000))?;
        self.file_data_offset = reader.stream_position()?;
        self.file_handle = Some(reader);
        Ok(())
    }

    /// Opens the HD1 file if it exists.
    fn open_hd1<T: AsRef<Path>>(&mut self, file_path: T) -> Result<()> {
        if self.header.hd1_delta != 0 {
            let hd1 = file_path.as_ref().with_extension("module_hd1");
            if hd1.exists() {
                self.use_hd1 = true;
                let file = File::open(hd1)?;
                self.hd1_file = Some(BufReader::new(file));
            }
        }
        Ok(())
    }

    /// Gets the tag path of a file entry.
    ///
    /// This function returns the tag path of a file entry based on the provided index.
    /// For file entries that have a parent, the function recursively gets the tag path of the parent and appends the child index to the path.
    ///
    /// # Arguments
    /// * `index` - The index of the file entry to get the tag path from.
    /// * `depth` - The depth of the recursion. This is used to prevent infinite recursion.
    ///
    /// # Returns
    /// Returns the tag path of the file entry if the operation is successful.
    fn get_tag_path(&self, index: usize, depth: usize) -> Result<String> {
        if depth > 3 {
            return Err(Error::TagError(TagError::RecursionDepth));
        }
        let file = &self.files[index];
        if file.tag_id == -1 && file.parent_index != -1 {
            let parent = &self.files[usize::try_from(file.parent_index)?];
            let mut parent_name: String = String::new();
            let child_index = self.resource_indices[usize::try_from(parent.resource_index)?
                ..usize::try_from(parent.resource_index)?
                    + usize::try_from(parent.resource_count)?]
                .iter()
                .map(|&i| &self.files[i as usize])
                .take_while(|&item| !eq(item, file))
                .count();
            if parent.tag_name.is_empty() {
                parent_name = self.get_tag_path(usize::try_from(file.parent_index)?, depth + 1)?;
            }
            if parent.tag_id == -1 {
                parent_name = self.get_tag_path(usize::try_from(file.parent_index)?, depth + 1)?;
                Ok(format!("{parent_name}[{child_index}:block]"))
            } else {
                Ok(format!("{parent_name}[{child_index}:resource]"))
            }
        } else {
            Ok(format!(
                "{}/{}.{}",
                file.tag_group, file.tag_id, file.tag_group
            ))
        }
    }

    /// Reads a specific tag from the module file.
    ///
    /// This function reads a specific tag from the module file based on the provided index.
    /// It also utilizes the HD1 stream if the file entry has the flag set for it and the stream is loaded, and returns `None` if the tag offset is invalid.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the file entry to read the tag from. This index corresponds to the position of the file entry in the [`files`](`ModuleFile::files`) vector.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the file if the read operation is successful, or an [`Error`](`crate::Error`), a [`None`] if the file was not read (if tag offset is specified as invalid) or the containing the I/O error if any reading operation fails.
    pub fn read_tag(&mut self, index: u32) -> Result<Option<&mut ModuleFileEntry>> {
        let file = &mut self.files[index as usize];
        if file.data_offset_flags.contains(DataOffsetType::DEBUG) {
            return Ok(None); // Currently not reading debug modules because we don't have an
            // example.
        }

        let mut offset = self.header.hd1_delta;
        if file.data_offset_flags.contains(DataOffsetType::USE_HD1) {
            if let Some(ref mut module_file) = self.hd1_file {
                if self.header.version <= ModuleVersion::CampaignFlight {
                    offset += self.header.hd1_delta;
                }
                file.read_tag(
                    module_file,
                    offset,
                    &self.blocks,
                    &self.header.version,
                    true,
                )?;
            } else {
                return Ok(None);
            }
        } else if let Some(ref mut module_file) = self.file_handle {
            file.read_tag(
                module_file,
                self.file_data_offset,
                &self.blocks,
                &self.header.version,
                false,
            )?;
        }
        Ok(Some(file))
    }

    /// Searches for the index of the tag given the `global_id`.
    ///
    /// This function searches for the index of a tag in the [`files`](`ModuleFile::files`) vector using the provided
    /// `global_id`. If the tag is found, it reads the tag using the [`read_tag`](`ModuleFile::read_tag`) function and
    /// stores it in the index.
    ///
    /// # Arguments
    ///
    /// * `global_id` - The global tag ID of the file to find. This ID is used to identify the specific tag within the module file.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the file if successful. If the tag is not
    /// found or couldn't be read, it returns [`None`]. Any I/O error encountered during the operation is also returned
    /// if it occurs.
    pub fn read_tag_from_id(&mut self, global_id: i32) -> Result<Option<&mut ModuleFileEntry>> {
        if let Some(index) = self.files.iter().position(|file| file.tag_id == global_id) {
            let has_read = self.read_tag(u32::try_from(index)?)?;
            if let Some(tag) = has_read {
                Ok(Some(tag))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
