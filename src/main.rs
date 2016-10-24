#[macro_use]
extern crate libloading;

use libloading::Symbol;
use libloading::Library;

fn main() {
    println!("Hello, world!");
    let lib  = Library::new("/root/IdeaProjects/rust-dynload/command/target/release/libembed.so").unwrap();
    unsafe {
        let awesome_function: Symbol<unsafe extern fn()> =
        lib.get(b"process\0").unwrap();
        awesome_function();
    }
}