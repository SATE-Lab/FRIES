use command::run_command;

mod command;
mod project;

fn main() {
    let f = |x| x + 1;
    let a = f(2);

    run_command();
}
