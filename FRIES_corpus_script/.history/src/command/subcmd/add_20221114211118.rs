/*
    This subcommand `add` is to add a target into fuzz target.
*/

use crate::command::RunCommand;

pub struct Add {}

impl RunCommand for Add {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
