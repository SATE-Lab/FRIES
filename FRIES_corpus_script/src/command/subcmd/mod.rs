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

mod gen;
mod init;
mod parse_dependents;
mod run;

pub use gen::Gen;
pub use init::Init;
pub use parse_dependents::ParseDependents;
pub use run::Run;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt, PartialEq)]
pub struct FuzzDirWrapper {
    #[structopt(long = "fuzz-dir")]
    pub fuzz_dir: Option<PathBuf>,
}
