/*
    This subcommand `add` is to add a target into fuzz target.
*/
use crate::command::RunCommand;
use anyhow::{Ok, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Add {}

impl RunCommand for Add {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
