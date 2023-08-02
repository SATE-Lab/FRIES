/*
    This is core module to implement the core functionility of the fuzzing target
*/
mod util;

use std::path::PathBuf;
use anyhow::Result;

use crate::command::subcmd::Init;

use self::util::find_this_package;

const DEFAULT_FUZZ_DIR: &str = "fuzz";

pub struct FuzzProject {
    fuzz_dir: PathBuf,
    target_dir: PathBuf,
    target_name: Vec<String>,
}

impl FuzzProject {
    fn init(init: &mut Init){
        let this_package_path = find_this_package()?;
        let fuzz_dir = match init.fuzz_dir_wrapper.fuzz_dir {
            Some(dir) => {}
            None => 
        };
    }
}
