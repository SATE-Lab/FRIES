/*
    This subcommand `run` is the engine to excute the fuzzing.
*/

#[derive(StructOpt)]
pub struct Run {}

impl RunCommand for Run {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
