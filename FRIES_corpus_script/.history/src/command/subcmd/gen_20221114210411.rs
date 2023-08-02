/*
    This subcommand is used to generate fuzzing target for Rust Library.
*/

#[derive(StructOpt)]
pub struct Gen {}

impl RunCommand for Gen {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
