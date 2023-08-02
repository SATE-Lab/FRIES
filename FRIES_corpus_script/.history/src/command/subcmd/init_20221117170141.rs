/*
    This subcommand init is to initialize the directory for fuzzing test.
*/
use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

use super::FuzzDirWrapper;

#[derive(StructOpt)]
pub struct Init {
    #[structopt(flatten)]
    pub fuzz_dir_wrapper: FuzzDirWrapper,

    #[structopt(short = "t", long = "target")]
    pub target_dir: Option<String>,
}

impl RunCommand for Init {
    fn run_command(&mut self) -> Result<()> {
        //TODO: complete this trait
        Ok(())
    }
}
