//! Module block containing info relating to Kraken compression.

use byteorder::{LE, ReadBytesExt};
use std::io::BufRead;

use crate::Result;
use crate::common::errors::ModuleError;
use crate::common::extensions::Enumerable;

#[derive(Default, Debug)]
/// Represents a module block entry containing information related to Kraken compression.
/// This struct is used to determine how to read bytes in [`ModuleFileEntry`](`super::file::ModuleFileEntry`).
pub(crate) struct ModuleBlockEntry {
    /// Offset in bytes of compressed data inside the module (after [`file_data_offset`](`super::loader::ModuleFile::file_data_offset`) in the module).
    pub(super) compressed_offset: u32,
    /// Size in bytes of compressed data inside the module.
    pub(super) compressed_size: u32,
    /// Offset in bytes of decompressed data inside the decompression buffer.
    pub(super) decompressed_offset: u32,
    /// Size in bytes of the decompression buffer.
    pub(super) decompressed_size: u32,
    /// Boolean indicating if the block is compressed or not.
    /// Tags can be made up of both compressed and decompressed blocks.
    pub(super) is_compressed: bool,
}

impl Enumerable for ModuleBlockEntry {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.compressed_offset = reader.read_u32::<LE>()?;
        self.compressed_size = reader.read_u32::<LE>()?;
        self.decompressed_offset = reader.read_u32::<LE>()?;
        self.decompressed_size = reader.read_u32::<LE>()?;
        let temp_compressed = reader.read_u32::<LE>()?;
        if temp_compressed != 0 && temp_compressed != 1 {
            return Err(ModuleError::IncorrectCompressedValue.into());
        }
        self.is_compressed = temp_compressed != 0;
        Ok(())
    }
}
