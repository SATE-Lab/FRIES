/*
    add a target into dir targets/
*/

use structopt::StructOpt


#[derive(StructOpt)]
pub struct Build {}

impl RunCommand for Build {
    fn run_command(&mut self) -> Result<()> {
        Ok(())
    }
}
