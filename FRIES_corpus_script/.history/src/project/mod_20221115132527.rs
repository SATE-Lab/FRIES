/*
    This is core module to implement the core functionility of the fuzzing target
*/

use std::path::{Path, PathBuf};

struct fuzz_project {
    fuzz_dir: PathBuf,
    target_dir: PathBuf,
    target_name: Vec<String>,
}
