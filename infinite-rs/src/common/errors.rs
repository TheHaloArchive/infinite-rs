//! Common errors used throughout `infinite-rs`.

use num_enum::TryFromPrimitiveError;
use std::io::Error as StdIoError;
use std::num::TryFromIntError;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;
use thiserror::Error;

use crate::{
    module::header::ModuleVersion,
    tag::{
        datablock::TagSectionType,
        structure::{TagStructLocation, TagStructType},
    },
};

#[derive(Error, Debug)]
/// Errors that can occur when reading a module file.
pub enum ModuleError {
    /// Incorrect magic number found in the module file header. Expected magic number is "ucsh" (0x64686F6D).
    #[error("Incorrect module magic found! Expected '0x64686F6D', found {0:#X}!")]
    IncorrectMagic(u32),
    /// Incorrect version number found in the module file header. Expected version is 53.
    /// While version 53 is the only fully supported version, other versions may also work.
    #[error("Incorrect module version found!")]
    IncorrectVersion(#[from] TryFromPrimitiveError<ModuleVersion>),
    /// Invalid negative block index found in module file, indicating file corruption.
    /// This error serves as a runtime assert.
    #[error("Module file block index must be non-negative, found {0}")]
    NegativeBlockIndex(i32),
    /// Occurs when the [`is_compressed`](`crate::module::block::ModuleBlockEntry::is_compressed`) value is not 0 or 1
    #[error("Value for is_compressed incorrect!")]
    IncorrectCompressedValue,
}

#[derive(Error, Debug)]
/// Errors that can occur when reading a tag file.
pub enum TagError {
    /// Incorrect magic number found in the tag file header. Expected magic number is "mohd" (0x68736375).
    #[error("Incorrect magic found! Expected '0x68736375', found {0:#X}!")]
    IncorrectMagic(u32),
    /// Incorrect version number found in the tag file header. Expected version is 27.
    /// Version 27 is used across all Infinite versions and matches Halo 5, though with different structures.
    #[error("Incorrect version found! Expected '27', found {0}!")]
    IncorrectVersion(i32),
    /// File data has not been loaded. Operations require [`data_stream`](`crate::module::file::ModuleFileEntry::data_stream`) to be initialized.
    #[error("Not been loaded yet!")]
    NotLoaded,
    /// Main struct designated by [`MainStruct`](`crate::tag::structure::TagStructType`) was not found in tag file.
    #[error("Main struct not found!")]
    MainStructNotFound,
    /// Tag metadata headers [`tag_info`](`crate::module::file::ModuleFileEntry::tag_info`) are missing.
    /// This occurs when attempting to read metadata from a [`RawFile`](`crate::module::file::FileEntryFlags::RAW_FILE`).
    #[error("Does not contain tag info!")]
    NoTagInfo,
    /// Failed to convert integer to [`TagSectionType`].
    /// This error should not occur as [`TagSectionType`] enum is exhaustive.
    #[error("Invalid TagStruct type encountered!")]
    InvalidTagSection(#[from] TryFromPrimitiveError<TagSectionType>),
    /// Failed to convert integer to [`TagStructType`].
    /// This error should not occur as [`TagStructType`] enum is exhaustive.
    #[error("Invalid TagStruct type encountered!")]
    InvalidTagStruct(#[from] TryFromPrimitiveError<TagStructType>),
    /// Failed to convert primitive to enum in [`common_types`](`crate::tag::types::common_types`).
    #[error("Failed to convert primitive to enum")]
    NumEnumError,
    /// Recursion depth reached 3 when trying to get tag path.
    /// This should never ever happen, if it has, something has gone very wrong.
    #[error("Recursion depth reached 3!")]
    RecursionDepth,
    /// Failed to convert integer to [`TagStructLocation`].
    /// This error should not occur as [`TagStructLocation`] enum is exhaustive.
    #[error("Invalid TagStruct location encountered!")]
    InvalidTagStructLocation(#[from] TryFromPrimitiveError<TagStructLocation>),
}

#[derive(Error, Debug)]
/// Errors that can occur when decompressing data.
pub enum DecompressionError {
    /// Buffer size is insufficient for decompressed data.
    /// This should not occur in Infinite module decompression.
    #[error("Buffer size overflow")]
    BufferSizeOverflow,
    /// Decompression failed with Kraken decompressor error code.
    /// Negative error codes indicate decompression failure.
    #[error("Decompression failed with error code {0}")]
    DecompressionFailed(i32),
}

#[derive(Error, Debug)]
/// Standard error type used throughout `infinite-rs`.
pub enum Error {
    /// IO error from [`std::io`] operations.
    #[error("Failed to read from buffer!")]
    ReadError(#[from] StdIoError),
    /// UTF-8 decoding error in [`read_fixed_string`](`crate::common::extensions::BufReaderExt::read_fixed_string`).
    #[error("Incorrect UTF-8 encoding found when reading string!")]
    Utf8ReadingError(#[from] FromUtf8Error),
    /// Kraken decompression error.
    #[error("Error occurred while decompressing!")]
    DecompressionError(#[from] DecompressionError),
    /// Module file loading error.
    #[error("Error occurred while loading a module!")]
    ModuleError(#[from] ModuleError),
    /// Integer type conversion error.
    #[error("Integer conversion failed!")]
    TryFromIntError(#[from] TryFromIntError),
    /// Tag file loading error.
    #[error("Error occurred while loading a tag!")]
    TagError(#[from] TagError),
}

/// Standard result type used throughout `infinite-rs`.
pub type Result<T> = StdResult<T, Error>;
