#[macro_use]
extern crate libloading;

use libloading::Symbol;
use libloading::Library;

extern crate shared;

use shared::Command;

fn main() {
    println!("Hello, world!");
    let command_chain: Vec<&Command> = vec![];
    let lib  = Library::new("/root/IdeaProjects/rust-dynload/command/target/debug/libembed.so").unwrap();
    unsafe {
        let submit: Symbol<unsafe extern fn()-> Box<Command> > =
        lib.get(b"submit\0").unwrap();
        let command_box = submit();
        let returned_box = unsafe{ *Box::into_raw(command_box)};
        command_chain.append(returned_box);
    }
}