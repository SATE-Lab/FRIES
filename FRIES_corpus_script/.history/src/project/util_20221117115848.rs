/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Context, Ok, Result};
use std::io::Write;
use std::{env, fs, path::PathBuf};

use core::result::Result::Ok;
use std::result::Result::Ok;

pub fn is_fuzz_cargo_toml(value: &mut toml::Value) -> bool {
    false
}

pub fn find_this_package() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    let mut data = Vec::new();
    loop {
        let cargo_toml_path = dir.join("Cargo.toml");

        match fs::File::open(&cargo_toml_path) {
            /*Ok(mut f) => {
                data.clear();

                //读取Cargo.toml内部的值
                f.read_to_end(&mut data).with_context(|| {
                    format!("failed to read {} toml file", cargo_toml_path.display())
                })?; //懒惰求值

                let value: toml::Value = toml::from_slice(&data).with_context(|| {
                    format!("failed to decode the file at {}", cargo_toml_path.display())
                })?;
                return Ok(dir);
            }*/
            Err(_) => {}
            Ok(_) => todo!(),
        }
        //如果空了，那就break
        if !dir.pop() {
            break;
        }
    }
    bail!(
        "We can't find the package according to this working dir:{}",
        env::current_dir().unwrap().display()
    );
}
