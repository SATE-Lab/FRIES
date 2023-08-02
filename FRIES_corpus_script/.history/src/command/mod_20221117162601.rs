/*
    This module is used to deal with command line instruction passed by the user.
*/

pub mod subcmd;

use anyhow::Result;

use structopt::StructOpt;

//统一接口，不同命令都统一使用run_command
trait RunCommand {
    fn run_command(&mut self) -> Result<()>;
}

#[derive(StructOpt)]
#[structopt(about = "This is a fuzzing test tool for Rust Library.")]
enum Command {
    ///Initialize the directory for fuzzing
    Init(subcmd::Init),

    ///Add a fuzzing target
    Add(subcmd::Add),

    ///Generate a fuzzing target
    // This is one of the most important parts of out project
    Gen(subcmd::Gen),

    ///Build the fuzz targets
    Build(subcmd::Build),

    ///Run the fuzz targets
    Run(subcmd::Run),
}

impl RunCommand for Command {
    fn run_command(&mut self) -> Result<()> {
        match self {
            Command::Init(x) => x.run_command(),
            Command::Add(x) => x.run_command(),
            Command::Gen(x) => x.run_command(),
            Command::Build(x) => x.run_command(),
            Command::Run(x) => x.run_command(),
        }
    }
}

pub fn run_command() {
    Command::from_args().run_command().unwrap();
}
