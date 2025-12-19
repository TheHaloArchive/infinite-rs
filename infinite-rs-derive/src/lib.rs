#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![warn(clippy::all)]

use std::collections::HashMap;

use quote::quote;
use syn::{DataStruct, DeriveInput};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(data))]
struct TagStructureAttributes {
    size: u64,
}

#[derive(deluxe::ExtractAttributes, Clone)]
#[deluxe(attributes(data))]
struct TagStructureFieldAttributes {
    offset: u64,
    count: Option<u64>,
}

fn extract_struct_field_attributes(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, TagStructureFieldAttributes>> {
    let mut field_attributes = HashMap::new();
    if let syn::Data::Struct(data) = &mut ast.data {
        for field in &mut data.fields {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attributes: TagStructureFieldAttributes = deluxe::extract_attributes(field)?;
            field_attributes.insert(field_name, attributes);
        }
    }
    Ok(field_attributes)
}

fn extract_field_maps(
    field_attributes: &HashMap<String, TagStructureFieldAttributes>,
) -> (Vec<String>, Vec<u64>) {
    field_attributes
        .clone()
        .into_iter()
        .map(|(field, attrs)| (field, attrs.offset))
        .unzip()
}

fn generate_field_reads(
    data: &DataStruct,
    field_attributes: &HashMap<String, TagStructureFieldAttributes>,
) -> Vec<proc_macro2::TokenStream> {
    data.fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let offset = field_attributes
                .get(&field_name.as_ref().unwrap().to_string())
                .unwrap()
                .offset;
            if let syn::Type::Path(type_path) = &field.ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "FieldArray" {
                        let count = field_attributes
                            .get(&field_name.as_ref().unwrap().to_string())
                            .unwrap()
                            .count
                            .unwrap();
                        return quote! {
                            reader.seek(std::io::SeekFrom::Start(main_offset + #offset))?;
                            self.#field_name.read(reader, #count)?;
                        };
                    }
                }
            }
            quote! {
                reader.seek(std::io::SeekFrom::Start(main_offset + #offset))?;
                self.#field_name.read(reader)?;
            }
        })
        .collect()
}

fn generate_field_blocks(
    data: &DataStruct,
    field_attributes: &HashMap<String, TagStructureFieldAttributes>,
) -> Vec<proc_macro2::TokenStream> {
    data.fields.iter().filter_map(|field| {
        if let syn::Type::Path(type_path) = &field.ty {
            if let Some(segment) = type_path.path.segments.last() {
                let field_name = &field.ident;
                match segment.ident.to_string().as_str() {
                    "FieldBlock" => {
                        let offset = field_attributes.get(&field_name.as_ref().unwrap().to_string()).unwrap().offset;
                        Some(quote! {
                            self.#field_name.load_blocks(source_index, adjusted_base + #offset, reader, tag_file)?;
                        })
                    },
                    "FieldTagResource" => {
                        let offset = field_attributes.get(&field_name.as_ref().unwrap().to_string()).unwrap().offset;
                        Some(quote! {
                            self.#field_name.load_resource(adjusted_base + #offset, reader, tag_file)?;
                        })
                    },
                    "FieldArray" => {
                        let offset = field_attributes.get(&field_name.as_ref().unwrap().to_string()).unwrap().offset;
                        Some(quote! {
                            self.#field_name.load_blocks(reader, source_index, adjusted_base + #offset, tag_file)?;
                        })
                    },
                    "FieldData" => {
                        Some(quote! {
                            self.#field_name.load_data(reader, source_index, parent_index, tag_file)?;
                        })
                    },
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}
fn tag_structure_derive2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input)?;
    let TagStructureAttributes { size } = deluxe::extract_attributes(&mut ast)?;
    let field_attributes: HashMap<String, TagStructureFieldAttributes> =
        extract_struct_field_attributes(&mut ast)?;
    let ident: &syn::Ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let syn::Data::Struct(data) = &ast.data else {
        panic!("TagStructure can only be derived for structs")
    };
    let (name, field_offset) = extract_field_maps(&field_attributes);

    let field_reads = generate_field_reads(data, &field_attributes);
    let field_blocks = generate_field_blocks(data, &field_attributes);

    Ok(quote! {
        impl #impl_generics infinite_rs::module::file::TagStructure for #ident #type_generics #where_clause {
            fn size(&mut self) -> u64 {
                #size
            }
            fn read<R: infinite_rs::common::extensions::BufReaderExt>(&mut self, reader: &mut R) -> infinite_rs::Result<()> {
                let main_offset = reader.stream_position()?;
                #(#field_reads)*
                reader.seek(std::io::SeekFrom::Start(main_offset + self.size()))?;
                Ok(())
            }

            fn offsets(&self) -> std::collections::HashMap<&'static str, u64> {
                let field_names = [#(#name),*];
                let field_offsets = [#(#field_offset),*];

                let map: std::collections::HashMap<&'static str, u64> = field_names.iter().zip(field_offsets.iter()).map(|(&name, &offset)| (name, offset)).collect();
                map
            }

            fn load_field_blocks<R: std::io::BufRead + std::io::Seek + infinite_rs::common::extensions::BufReaderExt>(
                &mut self,
                source_index: i32,
                parent_index: usize,
                adjusted_base: u64,
                reader: &mut R,
                tag_file: &infinite_rs::tag::loader::TagFile,
            ) -> infinite_rs::Result<()> {
                #(#field_blocks)*
                Ok(())
            }
        }
    })
}

#[proc_macro_derive(TagStructure, attributes(data))]
/// For implementing Tag Structures as described in documentation.
pub fn tag_structure_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tag_structure_derive2(input.into()).unwrap().into()
}
