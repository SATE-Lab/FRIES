/*
    This subcommand is used to generate fuzzing target for Rust Library.
*/

#[derive(StructOpt)]
pub struct GenTarget {}

impl RunCommand for GenTarget {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
