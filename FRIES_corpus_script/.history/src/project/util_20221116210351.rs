/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Result, Context};
use std::{env, fs, path::PathBuf, io::Read};

pub fn find_this_package() -> Result<PathBuf> {
    let mut cur_working_dir = env::current_dir()?;
    let mut data = Vec::new();
    loop {
        let cargo_toml_path = cur_working_dir.join("Cargo.toml");
        match fs::File::open(&cargo_toml_path) {
            Err(_) => {}
            Ok(mut f){
                data.clear();
                f.read_to_end(&mut data)
                    .with_context(||format!("failed to read {}", cargo_toml_path.display()));//懒惰求值
                let value: toml::Value = toml::from_slice(&data)
                    .with_context(||format!("failed to decode the file at {}",cargo_toml_path.display()));
            }
        }
    }
    bail!(
        "We can't find the package according to this working dir:{}",
        cur_working_dir
    );
}
