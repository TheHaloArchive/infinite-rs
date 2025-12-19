//! Hierarchical structure entry of tag.

use byteorder::{LE, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::io::BufRead;

use crate::Result;
use crate::common::errors::TagError;
use crate::common::extensions::Enumerable;

#[derive(Default, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(u16)]
/// Enum defining what the tag struct is pointing to.
pub enum TagStructType {
    #[default]
    /// "Root" of structure.
    MainStruct,
    /// An array of items in structure.
    TagBlock,
    /// Reference to child resource.
    Resource,
    /// Reference to "external" resource.
    Custom,
    /// Unknown
    Literal,
}

#[derive(Default, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(u16)]
/// Enum defining where teh data in the tag struct is pointing towards in a "Custom" tag structure.
pub enum TagStructLocation {
    #[default]
    Internal,
    Resource,
    Debug,
}

#[derive(Default, Debug)]
/// Structure defining the hierarchical order of info in tags.
pub struct TagStruct {
    /// GUID of the structure referenced.
    pub guid: u128,
    /// Where the structure is located.
    pub struct_type: TagStructType,
    /// Where the data for the structure is located.
    pub location: TagStructLocation,
    /// For main struct and tag block structs, the index of the block containing the struct.
    /// For resource structs, index of the resource.
    /// Can be -1 if the tag field doesn't point to anything.
    pub target_index: i32,
    /// The index of the data block containing the tag field which refers to this struct.
    /// Can be -1 for the main struct.
    pub field_block: i32,
    /// The offset of the tag field inside the data block.
    pub field_offset: u32,
}

impl Enumerable for TagStruct {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.guid = reader.read_u128::<LE>()?;
        self.struct_type = TagStructType::try_from(reader.read_u16::<LE>()?)
            .map_err(TagError::InvalidTagStruct)?;
        self.location = TagStructLocation::try_from(reader.read_u16::<LE>()?)
            .map_err(TagError::InvalidTagStructLocation)?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_i32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
