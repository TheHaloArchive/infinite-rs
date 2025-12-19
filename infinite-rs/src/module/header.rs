//! Module Header containing info on the layout of the module file.

use byteorder::{LE, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::{fs::File, io::BufReader};

use crate::Result;
use crate::common::errors::ModuleError;

const HEADER_MAGIC: u32 = 0x6468_6F6D; // "mohd"

#[derive(Default, Debug, PartialEq, Eq, TryFromPrimitive, PartialOrd, Ord)]
#[repr(i32)]
/// Revision number of a module file.
/// This version number determines how tags should be read.
pub enum ModuleVersion {
    /// First "technical preview" build from July 2021.
    Flight1 = 48,
    /// Second technical preview (August 2021) and release version from November 2021.
    Release = 51,
    /// Build used in the co-op campaign flight, which introduced notable changes to the module structure.
    CampaignFlight = 52,
    #[default]
    /// Builds from Season 3 and onwards.
    Season3 = 53,
}

#[derive(Default, Debug)]
/// Module Header structure containing info on the layout of the module file.
pub struct ModuleHeader {
    /// Should be "mohd" (0x64686F6D)
    magic: u32,
    /// Revision number of the module.
    /// This determines how offsets are calculated and if tag names should be read.
    pub version: ModuleVersion,
    /// Unique identifier of module.
    pub module_id: i64,
    /// Number of files in the module.
    pub file_count: u32,
    /// Index of `loadmanifest` tag, which contains the tag ids that the module will load.
    loadmanifest_index: i32,
    /// Index of `runtimeloadmetadata` tag, which contains info on how tags should be loaded at runtime.
    runtimeloadmetadata_index: i32,
    /// Index of `resourcemetadata` tag, which contains info on how resources should be loaded.
    resourcemetadata_index: i32,
    /// Index of the first resource entry ([`file_count`](`ModuleHeader::file_count`) - [`resource_count`](`ModuleHeader::resource_count`)).
    resource_index: i32,
    /// Total size in bytes of the string table.
    pub(super) strings_size: u32,
    /// Number of resource files.
    pub(super) resource_count: u32,
    /// Number of data blocks.
    pub(super) block_count: u32,
    /// Same between modules, changes per build?
    build_version: u64,
    /// If non-zero, requires hd1 file.
    pub(super) hd1_delta: u64,
    /// Total size of packed data in the module.
    /// Both compressed and uncompressed.
    /// Starts after files, blocks, and resources have been read.
    ///
    /// This does NOT apply for versions before [`ModuleVersion::Season3`].
    pub(super) data_size: u64,
}

impl ModuleHeader {
    /// Reads the module header from the given buffered reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
    ///
    /// # Errors
    /// - If the magic number is not equal to [`HEADER_MAGIC`] [`ModuleError::IncorrectMagic`]
    /// - If the version number is not recognized [`ModuleError::IncorrectVersion`]
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    pub(super) fn read(&mut self, reader: &mut BufReader<File>) -> Result<()> {
        self.magic = reader.read_u32::<LE>()?;
        if self.magic != HEADER_MAGIC {
            return Err(ModuleError::IncorrectMagic(self.magic).into());
        }
        self.version = ModuleVersion::try_from_primitive(reader.read_i32::<LE>()?)
            .map_err(ModuleError::IncorrectVersion)?;

        self.module_id = reader.read_i64::<LE>()?;
        self.file_count = reader.read_u32::<LE>()?;
        self.loadmanifest_index = reader.read_i32::<LE>()?;
        self.runtimeloadmetadata_index = reader.read_i32::<LE>()?;
        self.resourcemetadata_index = reader.read_i32::<LE>()?;
        self.resource_index = reader.read_i32::<LE>()?;
        self.strings_size = reader.read_u32::<LE>()?;
        self.resource_count = reader.read_u32::<LE>()?;
        self.block_count = reader.read_u32::<LE>()?;
        self.build_version = reader.read_u64::<LE>()?;
        self.hd1_delta = reader.read_u64::<LE>()?;
        self.data_size = reader.read_u64::<LE>()?;
        if self.version >= ModuleVersion::Release {
            reader.seek_relative(8)?; // Not needed for now.
        }
        Ok(())
    }
}
