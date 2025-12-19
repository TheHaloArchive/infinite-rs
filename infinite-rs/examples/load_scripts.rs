use std::{
    fs::File,
    io::{BufWriter, Write},
};

use infinite_rs::{ModuleFile, Result, tag::types::common_types::FieldData};
use infinite_rs_derive::TagStructure;

const DEPLOY_PATH: &str =
    "C:/XboxGames/Halo Infinite/Content/deploy/any/globals/globals-rtx-new.module";
const SAVE_PATH: &str = "./scripts";
const SCRIPT_GROUP: &str = "hsc*";

#[derive(Default, Debug, TagStructure)]
#[data(size(0x2D8))]
struct HsSourceFileTag {
    #[data(offset(0x294))]
    server: FieldData,
    #[data(offset(0x2AC))]
    client: FieldData,
}

fn main() -> Result<()> {
    let mut module = ModuleFile::from_path(DEPLOY_PATH)?;
    for idx in 0..module.files.len() {
        if module.files[idx].tag_group == SCRIPT_GROUP {
            let tag = module.read_tag(idx as u32)?;
            if let Some(tag) = tag {
                let source = tag.read_metadata::<HsSourceFileTag>()?;

                let server_buf = source.server.data;
                let client_buf = source.client.data;

                let server_file = File::create(format!("{SAVE_PATH}/{}_server.luac", tag.tag_id))?;
                let mut bw = BufWriter::new(server_file);
                bw.write_all(&server_buf)?;

                let client_file = File::create(format!("{SAVE_PATH}/{}_client.luac", tag.tag_id))?;
                let mut bw = BufWriter::new(client_file);
                bw.write_all(&client_buf)?;
            }
        }
    }
    Ok(())
}
