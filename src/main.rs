#[macro_use]
extern crate lazy_static;

mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

mod utils;

fn main() {
    day_one::main();
    day_two::main();
    day_three::main();
    day_four::main();
    day_five::main();
    day_six::main();
}
