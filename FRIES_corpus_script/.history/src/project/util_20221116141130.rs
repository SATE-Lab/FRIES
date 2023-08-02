/*
    There are tools for dealing with package finding and path spilting.
*/
use anyhow::Result;
use std::env;

pub fn find_this_package() -> Result<PathBuf> {
    let mutcur_working_dir = env::current_dir()?;
}
