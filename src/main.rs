// use crate
use crate::garden::vegetables::Asparagus;

// declare module
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
