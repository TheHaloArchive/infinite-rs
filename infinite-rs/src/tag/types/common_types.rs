//! Types used by the game to construct a tag.

use byteorder::{LE, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::{
    fmt::Debug,
    io::{BufRead, Seek, SeekFrom},
};

use crate::{
    Result, TagFile,
    common::errors::{Error, TagError},
    tag::{datablock::TagSectionType, structure::TagStructType},
};
use crate::{common::extensions::BufReaderExt, module::file::TagStructure};

#[derive(Default, Debug)]
/// _0: 32 Byte strings that usually store some sort of short name.
pub struct FieldString(pub String);

impl FieldString {
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_fixed_string(32)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1: 256 byte long string usually used to store paths.
pub struct FieldLongString(pub String);

impl FieldLongString {
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_fixed_string(256)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2: 32 bit unsigned integer containing a `MurmurHash3_x86_64` 32 bit value.
pub struct FieldStringId(pub i32);

impl FieldStringId {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _4: Signed integer type "char" in C.
pub struct FieldCharInteger(pub i8);

impl FieldCharInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _5: Signed integer type "short" in C.
pub struct FieldShortInteger(pub i16);

impl FieldShortInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _6: Signed integer type "long" in C.
pub struct FieldLongInteger(pub i32);

impl FieldLongInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _7: Signed integer type "__int64 (long long)" in C.
pub struct FieldInt64Integer(pub i64);

impl FieldInt64Integer {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i64::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _8: IEE 754 floating point number that stores an angle.
pub struct FieldAngle(pub f32);

impl FieldAngle {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _A: An unsigned "char" value in C used to calculate enums.
pub struct FieldCharEnum<T: num_enum::TryFromPrimitive<Primitive = u8>>(pub T);

impl<T: TryFromPrimitive<Primitive = u8>> FieldCharEnum<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::try_from_primitive(reader.read_u8()?)
            .map_err(|_| Error::TagError(TagError::NumEnumError))?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _B: An unsigned "short" value in C used to calculate enums.
pub struct FieldShortEnum<T: num_enum::TryFromPrimitive<Primitive = u16>>(pub T);

impl<T: TryFromPrimitive<Primitive = u16>> FieldShortEnum<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::try_from_primitive(reader.read_u16::<LE>()?)
            .map_err(|_| Error::TagError(TagError::NumEnumError))?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _C: An unsigned "long" value in C used to calculate enums.
pub struct FieldLongEnum<T: num_enum::TryFromPrimitive<Primitive = u32>>(pub T);

impl<T: num_enum::TryFromPrimitive<Primitive = u32>> FieldLongEnum<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::try_from_primitive(reader.read_u32::<LE>()?)
            .map_err(|_| Error::TagError(TagError::NumEnumError))?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _D: An unsigned "long" value in C used to calculate bitflags.
pub struct FieldLongFlags<T: bitflags::Flags<Bits = u32>>(pub T);

impl<T: bitflags::Flags<Bits = u32>> FieldLongFlags<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::from_bits_truncate(reader.read_u32::<LE>()?);
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _E: An unsigned "word (short)" value in C used to calculate bitflags.
pub struct FieldWordFlags<T: bitflags::Flags<Bits = u16>>(pub T);

impl<T: bitflags::Flags<Bits = u16>> FieldWordFlags<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::from_bits_truncate(reader.read_u16::<LE>()?);
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _F: An unsigned "byte (char)" value in C used to calculate bitflags.
pub struct FieldByteFlags<T: bitflags::Flags<Bits = u8>>(pub T);

impl<T: bitflags::Flags<Bits = u8>> FieldByteFlags<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = T::from_bits_truncate(reader.read_u8()?);
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _10: X and Y coordinates of a point in 2D.
pub struct FieldPoint2D {
    pub x: u16,
    pub y: u16,
}

impl FieldPoint2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_u16::<LE>()?;
        self.y = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _11:  X and Y coordinates of a rectangle in 2D.
pub struct FieldRectangle2D {
    pub x: u16,
    pub y: u16,
}

impl FieldRectangle2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_u16::<LE>()?;
        self.y = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _12: RGBA values of a color represented in u8.
/// Alpha value is unused.
pub struct FieldRGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // UNUSED
}

impl FieldRGBColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.r = reader.read_u8()?;
        self.g = reader.read_u8()?;
        self.b = reader.read_u8()?;
        self.a = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _13: RGBA values of a color represented in u8.
pub struct FieldARGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl FieldARGBColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.r = reader.read_u8()?;
        self.g = reader.read_u8()?;
        self.b = reader.read_u8()?;
        self.a = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _14: Real number represented as a float.
pub struct FieldReal(pub f32);

impl FieldReal {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _15: Real "fraction" value represented as a float.
pub struct FieldRealFraction(pub f32);

impl FieldRealFraction {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _16: X and Y coordinates of point in 2D stored as two floats.
pub struct FieldRealPoint2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealPoint2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _17: X, Y and Z coordinates of point in 3D stored as three floats.
pub struct FieldRealPoint3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealPoint3D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _18: X and Y coordinates of a vector in 2D stored as two floats.
pub struct FieldRealVector2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealVector2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _19: X, Y and Z coordinates of a vector in 3D stored as three floats.
pub struct FieldRealVector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealVector3D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1A: X, Y, Z and W values of a quaternion stored as four floats.
/// Used for rotation math.
pub struct FieldRealQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl FieldRealQuaternion {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        self.w = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1B: X and Y coordinates of a eular angle in 2D stored as two floats.
pub struct FieldRealEulerAngles2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealEulerAngles2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1C: X, Y and Z coordinates of a eular angle in 3D stored as two floats.
pub struct FieldRealEularAngles3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealEularAngles3D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1D: X, Y and D values of a plane in 2D stored as three floats.
pub struct FieldRealPlane2D {
    pub x: f32,
    pub y: f32,
    pub d: f32,
}

impl FieldRealPlane2D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.d = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1E: X, Y, Z and D values of a plane in 3D stored as four floats.
pub struct FieldRealPlane3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub d: f32,
}

impl FieldRealPlane3D {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        self.d = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1F: RGB values of a color stored as three floats.
pub struct FieldRealRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FieldRealRGBColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.r = reader.read_f32::<LE>()?;
        self.g = reader.read_f32::<LE>()?;
        self.b = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _20: RGBA values of a color stored as four floats.
pub struct FieldRealARGBColor {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FieldRealARGBColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.a = reader.read_f32::<LE>()?;
        self.r = reader.read_f32::<LE>()?;
        self.g = reader.read_f32::<LE>()?;
        self.b = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _21: HSV values of a color stored as a single float.
/// Unknown how the actual color is calculated
pub struct FieldRealHSVColor(f32);

impl FieldRealHSVColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _22: AHSV values of a color stored as a single float.
/// Unknown how the actual color is calculated
pub struct FieldRealAHSVColor(f32);

impl FieldRealAHSVColor {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _23: Minimum and Maximum bounds stored as two unsigned shorts in C (u16).
pub struct FieldShortBounds {
    pub min: u16,
    pub max: u16,
}

impl FieldShortBounds {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.min = reader.read_u16::<LE>()?;
        self.max = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _24: Minimum and Maximum angles stored as two floats.
pub struct FieldAngleBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldAngleBounds {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _25: Minimum and Maximum real values stored as two floats.
pub struct FieldRealBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldRealBounds {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _26: Minimum and Maximum real fraction values stored as two floats.
pub struct FieldRealFractionBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldRealFractionBounds {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _29: Long block flags, stored a 32-bit unsigned integer.
pub struct FieldLongBlockFlags(pub u32);

impl FieldLongBlockFlags {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2A: Word block flags, stored a 32-bit unsigned integer.
pub struct FieldWordBlockFlags(pub u32);

impl FieldWordBlockFlags {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2B: Byte block flags, stored a 32-bit unsigned integer.
pub struct FieldByteBlockFlags(pub u32);

impl FieldByteBlockFlags {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2C: Char block index, stores an 8-bit signed integer.
pub struct FieldCharBlockIndex(pub i8);

impl FieldCharBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2D: Custom char block index, stores an 8-bit signed integer.
pub struct FieldCustomCharBlockIndex(pub i8);

impl FieldCustomCharBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2E: Short block index, stores a 16-bit signed integer.
pub struct FieldShortBlockIndex(pub i16);

impl FieldShortBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2F: Custom short block index, stores a 16-bit signed integer.
pub struct FieldCustomShortBlockIndex(pub i16);

impl FieldCustomShortBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _30: Long block index, stores a 32-bit signed integer.
pub struct FieldLongBlockIndex(pub i32);

impl FieldLongBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _31: Custom long block index, stores a 32-bit signed integer.
pub struct FieldCustomLongBlockIndex(pub i32);

impl FieldCustomLongBlockIndex {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _34: Padding field, no data stored.
pub struct FieldPad;

impl FieldPad {
    pub fn read<R: Seek>(&mut self, reader: &mut R, length: u8) -> Result<()> {
        reader.seek_relative(i64::from(length))?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3C: Byte integer field, stores an 8-bit unsigned integer.
pub struct FieldByteInteger(pub u8);

impl FieldByteInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3D: Word integer field, stores a 16-bit unsigned integer.
pub struct FieldWordInteger(pub u16);

impl FieldWordInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3E: Dword integer field, stores a 32-bit unsigned integer.
pub struct FieldDwordInteger(pub u32);

impl FieldDwordInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3F: Qword integer field, stores a 64-bit unsigned integer.
pub struct FieldQwordInteger(pub u64);

impl FieldQwordInteger {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.0 = reader.read_u64::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _39: Array of structures stored in sequence.
pub struct FieldArray<T: TagStructure + Default> {
    pub elements: Vec<T>,
}

impl<T: TagStructure + Default> FieldArray<T> {
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R, size: u64) -> Result<()> {
        for _ in 0..size {
            let mut element = T::default();
            element.read(reader)?;
            self.elements.push(element);
        }
        Ok(())
    }

    pub fn load_blocks<R: BufReaderExt>(
        &mut self,
        reader: &mut R,
        source_index: i32,
        adjusted_base: u64,
        tag_file: &TagFile,
    ) -> Result<()> {
        for element in &mut self.elements {
            element.load_field_blocks(source_index, 0, adjusted_base, reader, tag_file)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _40: Tag block, stores the size of an array.
pub struct FieldBlock<T: TagStructure> {
    field_offset: u64,
    type_info: u64, // uintptr at runtime
    unknown: u64,   // uintptr at runtime
    pub size: u32,
    pub elements: Vec<T>,
}

impl<T: TagStructure + Debug + Default> FieldBlock<T> {
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.field_offset = reader.stream_position()?;
        self.type_info = reader.read_u64::<LE>()?;
        self.unknown = reader.read_u64::<LE>()?;
        self.size = reader.read_u32::<LE>()?;
        Ok(())
    }

    #[inline(never)]
    pub fn load_blocks<R: BufReaderExt>(
        &mut self,
        current_block: i32,
        collection_offset: u64,
        reader: &mut R,
        tag_file: &TagFile,
    ) -> Result<()> {
        // Empty blocks may cause issues.
        if self.size == 0 {
            return Ok(());
        }
        let structs = &tag_file.struct_definitions;
        let blocks = &tag_file.datablock_definitions;

        // This is the "root" of the tag block, pointing to where the metadata for it is stored.
        // If target index is -1, it's a resource block, which we don't want right now.
        let block_root = structs.iter().enumerate().find(|(_, s)| {
            s.field_block == current_block
                && u64::from(s.field_offset) == collection_offset
                && s.target_index != -1
        });

        if let Some(block_struct) = block_root {
            #[allow(clippy::cast_sign_loss)]
            let Some(block) = blocks.get(block_struct.1.target_index as usize) else {
                return Ok(());
            };

            let mut offset = block.offset;

            // HACK: Calculate offset using other blocks.
            let tagdata_size = blocks
                .iter()
                .filter(|x| x.section_type == TagSectionType::TagData)
                .map(|x| x.entry_size)
                .sum::<u32>();

            if block.section_type == TagSectionType::ResourceData {
                offset = block.offset + u64::from(tagdata_size);
            }
            let size = T::default().size();

            // We first read the object itself without any of its children
            reader.seek(SeekFrom::Start(offset))?;
            for _ in 0..self.size {
                let mut object = T::default();
                object.read(reader)?;
                self.elements.push(object);
            }

            // We then read the children, with the adjusted size parameter depending on the size.
            for (idx, element) in self.elements.iter_mut().enumerate() {
                let adjusted_base = size * idx as u64;
                element.load_field_blocks(
                    block_struct.1.target_index,
                    idx,
                    adjusted_base,
                    reader,
                    tag_file,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _41: Reference to an external tag.
pub struct FieldReference {
    type_info: u64, // uintptr at runtime
    pub global_id: i32,
    pub asset_id: u64,
    pub group: String,
    local_handle: i32,
}

impl FieldReference {
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.type_info = reader.read_u64::<LE>()?;
        self.global_id = reader.read_i32::<LE>()?;
        self.asset_id = reader.read_u64::<LE>()?;
        self.group = reader.read_fixed_string(4)?.chars().rev().collect(); // reverse string
        self.local_handle = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _42: "External" resource inside tag.
pub struct FieldData {
    data_pointer: u64, // uintptr at runtime
    type_info: u64,    // uintptr at runtime
    pub unknown: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

impl FieldData {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.data_pointer = reader.read_u64::<LE>()?;
        self.type_info = reader.read_u64::<LE>()?;
        self.unknown = reader.read_u32::<LE>()?;
        self.size = reader.read_u32::<LE>()?;
        Ok(())
    }

    pub fn load_data<R: BufReaderExt>(
        &mut self,
        reader: &mut R,
        parent_index: i32,
        parent_struct_index: usize,
        tag_file: &TagFile,
    ) -> Result<()> {
        let reference = tag_file
            .data_references
            .iter()
            .filter(|x| x.field_block == parent_index)
            .collect::<Vec<_>>();
        if let Some(reference) = reference.get(parent_struct_index) {
            if reference.target_index != -1 {
                let datablock = &tag_file
                    .datablock_definitions
                    .get(usize::try_from(reference.target_index)?);
                let position = reader.stream_position()?;
                if let Some(datablock) = datablock {
                    reader.seek(SeekFrom::Start(datablock.get_offset(tag_file)))?;
                    let mut buf = vec![0; self.size as usize];
                    reader.read_exact(&mut buf)?;
                    reader.seek(SeekFrom::Start(position))?;
                    self.data = buf;
                }
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
/// _43: Reference to tag resource.
pub struct FieldTagResource<T: TagStructure> {
    block: u64, // uintptr at runtime
    handle: u32,
    pub resource_index: u32,
    pub data: T,
}

impl<T: TagStructure + Debug> FieldTagResource<T> {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.block = reader.read_u64::<LE>()?;
        self.handle = reader.read_u32::<LE>()?;
        self.resource_index = reader.read_u32::<LE>()?;
        Ok(())
    }

    pub fn load_resource<R: BufReaderExt>(
        &mut self,
        adjusted_base: u64,
        reader: &mut R,
        tag_file: &TagFile,
    ) -> Result<()> {
        let resource = tag_file
            .struct_definitions
            .iter()
            .enumerate()
            .find(|(_, s)| {
                s.struct_type == TagStructType::Custom && u64::from(s.field_offset) == adjusted_base
            });
        if let Some(resource) = resource {
            let datablock = &tag_file
                .datablock_definitions
                .get(usize::try_from(resource.1.target_index)?);
            let position = reader.stream_position()?;
            if let Some(datablock) = datablock {
                let datablock_location = datablock.get_offset(tag_file);
                reader.seek(SeekFrom::Start(datablock_location))?;
                self.data.read(reader)?;
                self.data.load_field_blocks(
                    resource.1.target_index,
                    resource.0,
                    0,
                    reader,
                    tag_file,
                )?;
                reader.seek(SeekFrom::Start(position))?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
/// "Internal struct" of `AnyTag` field.
pub struct AnyTagGuts {
    pub tag_id: i32,
    pub local_tag_handle: i32,
}

impl AnyTagGuts {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.tag_id = reader.read_i32::<LE>()?;
        self.local_tag_handle = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// `AnyTag` is present in all non-resource tags.
/// Is used at runtime to calculate locations of tags in memory.
pub struct AnyTag {
    vtable_space: u64,
    pub internal_struct: AnyTagGuts,
}

impl AnyTag {
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.vtable_space = reader.read_u64::<LE>()?;
        self.internal_struct.read(reader)?;
        Ok(())
    }
}
