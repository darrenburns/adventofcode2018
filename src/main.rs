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

fn main() {
    println!("{:?}", ten::find_message());
}
