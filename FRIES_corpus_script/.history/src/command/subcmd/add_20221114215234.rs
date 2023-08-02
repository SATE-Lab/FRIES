/*
    This subcommand `add` is to add a target into fuzz target.
*/

use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Add {}

impl RunCommand for Add {
    fn run_command(&mut self) {}
}
