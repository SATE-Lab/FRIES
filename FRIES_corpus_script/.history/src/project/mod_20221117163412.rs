/*
    This is core module to implement the core functionility of the fuzzing target
*/
mod util;

use std::path::PathBuf;

use crate::command::subcmd::Init;

const DEFAULT_FUZZ_DIR: &str = "fuzz";



pub struct FuzzProject {
    fuzz_dir: PathBuf,
    target_dir: PathBuf,
    target_name: Vec<String>,
}

impl FuzzProject {
    fn init(init: &mut Init) {
        let fuzz_dir = match init.fuzz_dir_wrapper.fuzz_dir {
            Some(dir)=>{}
            None=> 
        };
    }
}
