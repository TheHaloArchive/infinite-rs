# infinite-rs Changelog

## 0.13.0 - 2025-12-18
- Data blocks can now contain `FieldBlock` and are now read properly.
- `read_metadata` now takes in a generic `TagStructure` and returns the read result.
- `IncorrectCompressedValue` error added

## 0.12.1 - 2025-03-04
- Fixed issue that caused HD1 files not to be read.

## 0.12.0 - 2025-02-24
- Fix reading of `FieldBlock` on resource files.
- Update to Rust 2024
- Simplified handling of field reads.

## 0.11.0 - 2025-02-20
- `FieldData` now reads the data contained inside, and can be accessed using the `data` member.
- `block` in `FieldTagResource` is now private.

## 0.10.1 - 2025-02-10
- Added `FieldArray` which stores a sequence of a TagStructure in sequence.
- Added `count` field attribute for TagStructure. 

## 0.10.0 - 2025-02-09
- `FieldTagResource` now accepts a generic type that will be read.

## 0.9.0 - 2025-02-07
- Added `TagStructLocation` to `TagStruct`
- `FieldCharBlockIndex`, `FieldCustomCharBlockIndex`, `FieldShortBlockIndex`, `FieldCustomShortBlockIndex`, `FieldLongBlockIndex`, `FieldCustomLongBlockIndex` are now signed types.
- Fixed issue with reading blocks that don't start at the first datablock.

## 0.8.4 - 2025-02-05
- Fixed resource sections of tags not being read properly.

## 0.8.3 - 2025-01-07
- Some fixes to the `derive` feature.

## 0.8.2 - 2025-01-07
- Added `get_raw_data` to `ModuleFile` for easier access to data.
- Updated dependencies.
- `TagStructure` procmacro is now re-exported by infinite-rs if `derive` feature is enabled.

## 0.8.1 - 2024-12-17
- Fixed issues assigning tag names to tags in post-season 3 modules.

## 0.8.0 - 2024-12-17
- Many improvements to error documentation.
- `DataOffsetType::INVALID` is now `DataOffsetType::DEBUG`
- `read_compressed_block` and `decompress` are now marked as unsafe.
- `manifest0_count`, `manifest1_count`, `manifest2_count` have been renamed to `loadmanifest_index`, `runtimeloadmetadata_index` and `resourcemetadata_index`.
- `read_tag` and `read_tag_from_id` now return a mutable reference to the file if successful.

## 0.7.3 - 2024-12-15
- Fixed edge case where `psod` tags would fail to read dependency list. 

## 0.7.2 - 2024-12-13
- `ModuleFileEntry` now reads tag info if file is not raw.
- `read_tag` and `read_tag_from_id` now return `None` if the file is to be read from HD1 and the HD1 stream is not available.
- `TagDependency` and `TagReference` now have `name` fields read from a tag's string table before season 3.
- `tag_name` field in `ModuleFileEntry` now represents resource files and extensions properly for post-season 3 modules.

## 0.7.1 - 2024-12-13
- Made `header` field of `ModuleFile` public.

## 0.7.0 - 2024-12-11
- Added support for Module versions back to first Technical Preview
- `ModuleFileEntry` now has a `tag_name` field, filled by either the tag name found in its string list or its tag id. 
- Zonesets have been removed.
- Miscellaneous internal fixes.
- `ModuleFile::read_tag` now returns `Option<i32` containing the global tag id of the file being read.

## 0.6.2 - 2024-12-10
- Made some fields in `TagHeader` public.
- Added example `load_scripts`.

## 0.6.1 - 2024-12-03
- Added `PartialEq` and `Eq` implementations for `DataOffsetType`, `FieldEntryFlags` and `TagSectionType`.

## 0.6.0 - 2024-11-29
- FIX: HD1 tags are now properly identified.
- Multiple fields in `ModuleFileEntry` have been made public.
- Added `use_hd1` field to `ModuleFile`
- Removed `read_resources` from `ModuleFile`.

## 0.5.3 - 2024-11-22
- Internal: Each block does not create its own `BufReader` anymore.
- Fixed major issue affecting `FieldBlock` reads.

## 0.5.2 - 2024-11-21
- All tuple-like field types in `common_types` are now public.

## 0.5.1 - 2024-11-21
- Fixed docs.rs issues.

## 0.5.0 - 2024-11-21
- Many documentation items have been improved and inner links added
- Internal: `Readable` has been renamed to `Enumerable`
- Internal: Removed unnecessary trait bounds
- A `ModuleFile` can now be instantiated using `from_path`
- `Error` is now a crate-level export.
- Added support for enums and bitflags for common types
- Removed unused common types
- Many primitive common types now are tuple-like structs
- Reduced allocations with `read_enumerables` and Kraken decompressor
- Kraken decompressor has been vendored, now does not include large `oodle.txt` file.
- Updated dependencies.

## 0.4.2 - 2024-11-19
-  `module.read()` now supports any `AsRef<Path>` as a filepath argument
- The `datablock` and `structs` fields in `TagFile` have been renamed to `datablock_definitions` and `struct_definitions`.
- In `ModuleFile`, the `file_path` field has been removed, and `resources` renamed to `resource_indices`.
- Warning about `ds` modules has been removed.
- `AnyTag` now reads its contents directly

## 0.4.1 - 2024-11-19
- Fixed Kraken decompressor not working on windows
- Added new error types for type conversions
- Added github CI.

## 0.4.0 - 2024-11-18
- Kraken decompressor can now build and link on linux
- Added support for loading custom tag structures
- Added new derive macro crate to generate tag structures
- Field blocks can now be read properly
- Miscellaneous changes and improvements.

## 0.3.1 - 2024-10-21
- Resource tags can now be read using ModuleFile::read_tag.

## 0.3.0 - 2024-10-21
- Reworked build script to run on linux
- Updated dependencies
- Added `serde` feature to enable serialization for custom structs
- Implemented proper error types instead of using `anyhow`
- Changed `read_fixed_string` to not allow non-UTF8 characters
- Added visibility modifiers to most structs and functions
- Made public API more concise
- Updated documentation
- Changed modules to not do a syscall by keeping file stream in memory
- Added support for HD1 modules
- Removed `types` module, providing an example in `load_all_modules` instead.
## 0.2.2 - 2024-09-09
- Minor changes, implemented `Readable` type for other enumerables.
## 0.2.1 - 2024-09-09
- Added `read_enumerables` for BufReaderExt
- Updated documentation.
## 0.2.0 - 2024-09-05
- Fixed module items not being read correctly.
## 0.1.0 - 2024-08-26
- Initial Release
