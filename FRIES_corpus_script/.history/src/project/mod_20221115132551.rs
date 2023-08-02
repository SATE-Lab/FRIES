/*
    This is core module to implement the core functionility of the fuzzing target
*/

use std::path::{Path, PathBuf};

pub struct FuzzProject {
    fuzz_dir: PathBuf,
    target_dir: PathBuf,
    target_name: Vec<String>,
}
