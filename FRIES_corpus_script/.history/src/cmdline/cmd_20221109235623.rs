/*
    This is the entry of the fuzzer as a command line tool.
*/

use anyhow::Result;

pub use super::options;

const HELP_INFO: &str = "Fuzz rust";
const INVALID_CMD: &str = "\
INVALID_CMD: {cmd}
    {cause}
    {HELP_INFO}
";

trait RunCommand {
    fn run_command(&mut self) -> Result<()>;
}

enum Command {
    Init(),
    Add(),
}
