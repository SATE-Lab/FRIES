/*
    add a target into dir targets/
*/
use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::{BuildOptions, RunCommand};

#[derive(StructOpt)]
pub struct Build {
    #[structopt(flatten)]
    pub build_option: BuildOptions,
}

impl RunCommand for Build {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
