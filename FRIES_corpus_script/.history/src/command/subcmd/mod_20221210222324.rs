/*
    This module is designed to parse subcommand and run them.
    One subcommand is in one mod.

    By now, there are
    1. `init` to initialize the directory for fuzzing test
    2. `add` to add fuzzing target in the test directory
    3. `gen` to generate fuzzing target for library.
    4. `build` to build the target.
    5. `run` to excute fuzzing test.
*/

mod add;
mod build;
mod gen;
mod get_dependents;
mod init;
mod run;

use std::path::PathBuf;

pub use add::Add;
pub use build::Build;
pub use gen::Gen;
pub use get_dependents::GetDependents;
pub use init::Init;
pub use run::Run;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt, PartialEq)]
pub struct FuzzDirWrapper {
    #[structopt(long = "fuzz-dir")]
    pub fuzz_dir: Option<PathBuf>,
}
