/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::Result;
use std::{env, path::PathBuf};

pub fn find_this_package() -> Result<PathBuf> {
    let mut cur_working_dir = env::current_dir()?;
}
