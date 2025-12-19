infinite-rs
===
Simple and fast deserialization library for Halo Infinite.

This crate currently is in early-development. Please let me know via Github issues about any issues you encounter using this project.

[![Crates.io](https://img.shields.io/crates/v/infinite-rs.svg)](https://crates.io/crates/infinite-rs)
[![Documentation](https://docs.rs/infinite-rs/badge.svg)](https://docs.rs/infinite-rs)
![License](https://img.shields.io/crates/l/infinite-rs.svg)

## Documentation
- Documentation on this project can be found at [docs.rs](https://docs.rs/infinite-rs).

## Examples/Usage

### Getting Started: Loading a Module file
Modules are the file format that store "tags" in Halo Infinite. These files are used to store all the assets in the game, including models, textures, metadata, and more. `infinite-rs` provides a simple interface to load these tags, starting with loading the module files themselves.

```rust
use infinite_rs::{ModuleFile, Result};

fn load_modules() -> Result<()> {
    // Reads to the module file given a file path.
    // These are the main archive files used in Halo Infinite.
    // Note: the path can be anything that implements AsRef<Path>.
    let mut module = ModuleFile::from_path("C:/XboxGames/Halo Infinite/Content/deploy/any/globals-rtx-new.module")?;
    Ok(())
}
```

### Loading a tag file
After we have loaded a module file, we can now use the `read_tag` function to load a specific tag by index from the module file. This populates the `data_stream` and `tag_info` properties in a module entry that we can use later.

The `read_tag_from_id` function is also available to load a tag by its global ID.

```rust
use infinite_rs::{ModuleFile, Result};

fn load_tags() -> Result<()> {
    let mut module = ModuleFile::from_path("C:/XboxGames/Halo Infinite/Content/deploy/any/globals-rtx-new.module")?;

    // Load a specific tag from the module file.
    let tag_index = 0;
    let tag = module.read_tag(tag_index)?;
    // We can now access the data stream and tag info.
    if let Some(tag) = tag {
        // We can now access the data stream and tag info.
        let tag_data = tag.data_stream.as_ref().unwrap();
        let tag_info = tag.tag_info.as_ref().unwrap();
    }
    Ok(())
}
```

### Creating a custom structure and reading it
`infinite-rs` also allows you to read data directly into structures, using the `read_metadata` function. This functionality requires the `derive` feature.

#### Defining Structures
To define a structure that can be read from a tag data stream, you must first derive the `TagStructure` trait. To ensure proper padding and alignment, you can use the `data` attribute to specify the size of the structure in bytes. Each field also must contain a `data` attribute specifying the offset in bytes from the start of the structure.

> [!TIP]
> Padding between fields are automatically calculated. Any data between two offsets are skipped.

```rust
use infinite_rs::tag::types::common_types::{
    AnyTag, FieldReference,
};
use infinite_rs::TagStructure;

#[derive(Default, Debug, TagStructure)]
#[data(size(0x88))] // Size can be any u64 value.
struct MaterialTag {
    #[data(offset(0x00))] // Offset can be any u64 value within the range of the size.
    any_tag: AnyTag,
    #[data(offset(0x10))]
    material_shader: FieldReference,
}
```

#### Reading structures

```rust
use infinite_rs::tag::types::common_types::{
    AnyTag, FieldReference,
};
use infinite_rs::{ModuleFile, Result, TagStructure};

#[derive(Default, Debug, TagStructure)]
#[data(size(0x88))] // Size can be any u64 value.
struct MaterialTag {
    #[data(offset(0x00))] // Offset can be any u64 value within the range of the size.
    any_tag: AnyTag,
    #[data(offset(0x10))]
    material_shader: FieldReference,
}

fn load_tags() -> Result<()> {
    let mut module = ModuleFile::from_path("C:/XboxGames/Halo Infinite/Content/deploy/any/globals-rtx-new.module")?;

    // We now want to find the material tags in the module file.
    let material_indices = module.files.iter()
        .enumerate()
        .filter(|(_, file)| file.tag_group == "mat ")
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    // And for each material tag, we want to read the metadata associated.
    for index in material_indices {
        // We first have to populate data_stream and tag_info.
        let tag = module.read_tag(index as u32)?;
        if let Some(tag) = tag {
            let mat = tag.read_metadata::<MaterialTag>()?;
            // We can now access the fields in our structure.
            // For instance, `any_tag.internal_struct.tag_id` is always equal to the tag id of our file.
            assert_eq!(tag.tag_id, mat.any_tag.internal_struct.tag_id);
        }
    }    
    Ok(())
}
```

#### Reading enums and flags
`infinite-rs` also supports the usage of enums and flags as fields, available on the common types: `FieldCharEnum`, `FieldShortEnum`, `FieldLongEnum`, `FieldLongFlags`, `FieldWordFlags` and `FieldByteFlags`.

For enums, this requires `TryFromPrimitive` to be implemented.
For flags, you can use the `bitflags` crate.

```rust
use infinite_rs::tag::types::common_types::{FieldShortEnum, FieldWordFlags};
use infinite_rs::TagStructure;
use num_enum::TryFromPrimitive;
use bitflags::bitflags;

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
enum Variants {
    #[default]
    One,
    Two,
    Three
}

bitflags! {
    #[derive(Default, Debug)]
    struct FlagVariants : u16 {
        const ONE = 0b00;
        const TWO = 0b01;
        const THREE = 0b10;
    }
}

#[derive(Default, Debug, TagStructure)]
#[data(size(16))]
struct ExampleStruct {
    #[data(offset(0))]
    variants: FieldShortEnum<Variants>,
    #[data(offset(2))]
    variant_flags: FieldWordFlags<FlagVariants>
}
```

## Credits
- [libinfinite](https://github.com/Coreforge/libInfinite) by Coreforge, which this project is mostly based on.
- [Reclaimer](https://github.com/Gravemind2401/Reclaimer) by Gravemind2401, which helped me get familiar with Blam file formats.
- [AusarDocs](https://github.com/ElDewrito/AusarDocs) by Shockfire, a very useful resource on Ausar/Slipspace file formats.
- [Kraken](https://github.com/WolvenKit/kraken) by Wolvenkit team, a re-implementation of Oodle Kraken, removing the need for any binary blobs being required for decompression.
- [TagFramework](https://github.com/Codename-Atriox/TagFramework) by Codename Atriox, which was a common reference point for Slipspace internals.
- [red4lib](https://github.com/rfuzzo/red4lib) by rfuzzo, acting as the main inspiration for this project.
- [HIRT](https://github.com/urium1186/HIRT) by urium1186, which was very useful in debugging and verifying output from this project.
