use anyhow::{Ok, Result};

use structopt::StructOpt;

use crate::command::RunCommand;

#[derive(StructOpt)]
pub struct Init {}

impl RunCommand for Init {
    fn run_command(&mut self) -> Result<()> {
        //TODO: complete this trait
        Ok(())
    }
}
