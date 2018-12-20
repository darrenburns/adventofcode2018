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
mod twelve;

fn main() {
    println!("{:?}", twelve::get_living_plant_numbers_summed());
}
