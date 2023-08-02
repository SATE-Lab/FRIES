mod command;
//mod project;

use command::run_command;

#[macro_use]
extern crate log;

fn main() {
    run_command();
}
