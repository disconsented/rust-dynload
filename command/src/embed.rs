extern crate shared;
use std::result;

#[no_mangle]
pub extern fn submit() -> Command{
    println!("Hello, world from the lib!");
    Command("thing")
}