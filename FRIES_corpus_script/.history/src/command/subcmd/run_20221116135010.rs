/*
    This subcommand `run` is the engine to excute the fuzzing.
*/

use anyhow::Ok;
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Run {}

impl RunCommand for Run {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
