use command::run_command;

mod command;
mod project;

fn main() {
    let f = |x| -> i32 { x + 1 };
    let x = f(10);
    run_command();
}
