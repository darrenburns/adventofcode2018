#[macro_use] extern crate scan_fmt;

mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;

fn main() {
    println!("{:?}", eleven::get_optimal_cell_coords());
}
