/*
    This subcommand `run` is the engine to excute the fuzzing.
*/

use crate::command::RunCommand;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Run {}

impl RunCommand for Run {
    fn run_command(&mut self) {}
}
