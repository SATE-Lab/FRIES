use command::run_command;

mod command;
mod project;

fn main() {
    let x = |x| x + 2;
    run_command();
}
