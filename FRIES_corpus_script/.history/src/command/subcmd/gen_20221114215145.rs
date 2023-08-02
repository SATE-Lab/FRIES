/*
    This subcommand is used to generate fuzzing target for Rust Library.
*/

use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Gen {}

impl RunCommand for Gen {
    fn run_command(&mut self) {
        Ok(())
    }
}
