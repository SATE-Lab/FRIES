use crate::cmdline::cmd::RunCommand;
use anyhow::{Ok, Result};
pub struct Init {}

impl RunCommand for Init {
    fn run_command(&mut self) -> Result<()> {
        //TODO: complete this trait
        Ok(1)
    }
}
