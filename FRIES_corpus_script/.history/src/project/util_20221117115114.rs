/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Context, Ok, Result};
use std::{env, fs, path::PathBuf};

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
            Ok(mut f) => {}
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
