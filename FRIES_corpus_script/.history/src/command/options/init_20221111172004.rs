use super::super::RunCommand;
use anyhow::{Ok, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Init {}

impl RunCommand for Init {
    fn run_command(&mut self) -> Result<()> {
        //TODO: complete this trait
        Ok(())
    }
}
