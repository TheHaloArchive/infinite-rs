use std::path::Path;

use bitflags::bitflags;
use infinite_rs::tag::types::common_types::{
    AnyTag, FieldBlock, FieldByteFlags, FieldCharEnum, FieldLongEnum, FieldReference, FieldStringId,
};
use infinite_rs::{ModuleFile, Result};
use infinite_rs_derive::TagStructure;
use num_enum::TryFromPrimitive;

fn load_modules<R: AsRef<Path>>(deploy_path: R) -> Result<Vec<ModuleFile>> {
    let mut modules = Vec::new();
    for entry in walkdir::WalkDir::new(deploy_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap();
            if file_path.ends_with(".module") {
                let module = ModuleFile::from_path(file_path)?;
                modules.push(module);
            }
        }
    }
    Ok(modules)
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x30))]
struct MaterialShaderFunctionParameter {
    #[data(offset(0x04))]
    input_name: FieldStringId,
}

bitflags! {
    #[derive(Default, Debug)]
    pub struct MaterialFlags : u8 {
        const CONVERTED_FROM_SHADER = 0b01;
        const DECAL_POST_LIGHTING = 0b10;
        const RUNTIME_GENERATED = 0b100;
    }
}

#[derive(TryFromPrimitive, Debug, Default)]
#[repr(u32)]
enum MaterialParameterType {
    #[default]
    Bitmap,
    Real,
    Int,
    Bool,
    Color,
    ScalarGPUProperty,
    ColorGPUProperty,
    String,
    Preset,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x9c))]
struct MaterialParameter {
    #[data(offset(0x8))]
    bitmap: FieldReference,
    #[data(offset(0x4))]
    parameter_type: FieldLongEnum<MaterialParameterType>,
    #[data(offset(0x80))]
    function_parameters: FieldBlock<MaterialShaderFunctionParameter>,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x38))]
struct MaterialPostprocessTexture {
    #[data(offset(0x00))]
    texture: FieldReference,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0xA0))]
struct PostProcessDefinition {
    #[data(offset(0x00))]
    textures: FieldBlock<MaterialPostprocessTexture>,
}

#[derive(TryFromPrimitive, Debug, Default)]
#[repr(u8)]
enum MaterialStyleShaderSupportedLayers {
    #[default]
    Supports1Layer,
    Supports4Layers,
    Supports7Layers,
    LayerShaderDisabled,
}

#[derive(TryFromPrimitive, Debug, Default)]
#[repr(u8)]
enum MaterialStyleShaderSupportsDamageEnum {
    #[default]
    No,
    Yes,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x5c))]
struct MaterialStyleInfo {
    #[data(offset(0x00))]
    material_style: FieldReference,
    #[data(offset(0x1C))]
    material_style_tag: FieldReference,
    #[data(offset(0x38))]
    region_name: FieldStringId,
    #[data(offset(0x3C))]
    base_intention: FieldStringId,
    #[data(offset(0x40))]
    mask0_red_channel_intention: FieldStringId,
    #[data(offset(0x44))]
    mask0_green_channel_intention: FieldStringId,
    #[data(offset(0x48))]
    mask0_blue_channel_intention: FieldStringId,
    #[data(offset(0x4C))]
    mask1_red_channel_intention: FieldStringId,
    #[data(offset(0x50))]
    mask1_green_channel_intention: FieldStringId,
    #[data(offset(0x54))]
    mask1_blue_channel_intention: FieldStringId,
    #[data(offset(0x58))]
    supported_layers: FieldCharEnum<MaterialStyleShaderSupportedLayers>,
    #[data(offset(0x59))]
    requires_damage: FieldCharEnum<MaterialStyleShaderSupportsDamageEnum>,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x88))]
struct MaterialTag {
    #[data(offset(0x00))]
    any_tag: AnyTag,
    #[data(offset(0x10))]
    material_shader: FieldReference,
    #[data(offset(0x2C))]
    material_parameters: FieldBlock<MaterialParameter>,
    #[data(offset(0x40))]
    postprocess_definition: FieldBlock<PostProcessDefinition>,
    #[data(offset(0x6A))]
    flags: FieldByteFlags<MaterialFlags>,
    #[data(offset(0x74))]
    style_info: FieldBlock<MaterialStyleInfo>,
}

fn main() -> Result<()> {
    let mut modules = load_modules(String::from("C:/XboxGames/Halo Infinite/Content/deploy/"))?;

    for module in &mut modules {
        for index in 0..module.files.len() {
            let tag = module.read_tag(index as u32)?;
            if let Some(tag) = tag {
                if tag.tag_group == "mat " {
                    let _ = tag.read_metadata::<MaterialTag>()?;
                }
                // explicitly drop buffer to free up memory
                // normally, can take 50+ GBs of RAM
                module.files[index].data_stream = None
            }
        }
    }
    Ok(())
}
