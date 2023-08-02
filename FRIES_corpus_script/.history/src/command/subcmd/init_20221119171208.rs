/*
    This subcommand init is to initialize the directory for fuzzing test.
*/

use super::FuzzDirWrapper;
use crate::command::RunCommand;
use anyhow::{Ok, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Init {
    #[structopt(flatten)]
    pub fuzz_dir_wrapper: FuzzDirWrapper,

    #[structopt(short = "t", long = "target")]
    pub target_dir: Option<PathBuf>,
}

impl RunCommand for Init {
    fn run_command(&mut self) -> Result<()> {
        //TODO: complete this trait

        Ok(())
    }
}
