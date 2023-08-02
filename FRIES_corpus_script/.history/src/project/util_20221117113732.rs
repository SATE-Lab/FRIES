/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Context, Result};
use std::{env, fs, io::Read, path::PathBuf};

pub fn is_fuzz_cargo_toml(value: &mut toml::Value) -> bool {
    false
}

pub fn find_this_package() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    let mut data = Vec::new();
    loop {
        let cargo_toml_path = dir.join("Cargo.toml");

        match fs::File::open(&cargo_toml_path) {
            Err(_) => {}
            Ok(mut f) => {
                data.clear();
                //读取Cargo.toml内部的值
                f.read_to_end(&mut data)
                    .with_context(|| format!("failed to read {}", cargo_toml_path.display()))?; //懒惰求值
                let value: toml::Value = toml::from_slice(&data).with_context(|| {
                    format!("failed to decode the file at {}", cargo_toml_path.display())
                })?;
                
                break;
            }
        }
        if !
    }
    bail!(
        "We can't find the package according to this working dir:{}",
        cur_working_dir.display()
    );
}
