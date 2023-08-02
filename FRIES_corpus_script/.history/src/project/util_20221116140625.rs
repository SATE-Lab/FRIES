/*
    There are tools for dealing with package finding and path spilting.
*/

use anyhow::Result;

pub fn find_this_package() -> Result<PathBuf> {
    let cur_working_dir = std::env::current_dir()?;
}
