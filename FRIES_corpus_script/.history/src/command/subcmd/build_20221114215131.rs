/*
    add a target into dir targets/
*/
use anyhow::{Ok, Result};
use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Build {}

impl RunCommand for Build {
    fn run_command(&mut self) {
        Ok(())
    }
}
