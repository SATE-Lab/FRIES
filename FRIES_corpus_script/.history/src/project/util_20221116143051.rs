/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::{bail, Result};
use std::{env, path::PathBuf};

pub fn find_this_package() -> Result<PathBuf> {
    let mut cur_working_dir = env::current_dir()?;
    let mut data = Vec::new();
    loop {
        let cargo_toml_path = cur_working_dir.join("Cargo.toml");
    }
    bail!(
        "We can't find the package according to this working dir:{}",
        cur_working_dir
    );
}
