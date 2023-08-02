/*
    This subcommand `add` is to add a target into fuzz target.
*/

#[derive(StructOpt)]

pub struct GenTarget {}

impl RunCommand for GenTarget {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
