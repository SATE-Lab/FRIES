/*
    add a target into dir targets/
*/

use crate::command::{BuildOptions, RunCommand};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Build {
    #[structopt(flatten)]
    pub build_option: BuildOptions,
}

impl RunCommand for Build {
    fn run_command(&mut self) {}
}
