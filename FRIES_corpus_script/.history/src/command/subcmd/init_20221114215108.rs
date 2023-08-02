/*
    This subcommand init is to initialize the directory for fuzzing test.
*/

use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Init {}

impl RunCommand for Init {
    fn run_command(&mut self) {
        //TODO: complete this trait
        Ok(())
    }
}
