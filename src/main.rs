#[macro_use]
extern crate libloading;

use libloading::Symbol;
use libloading::Library;

use std::io;

extern crate shared;

use shared::Command;

fn get_command(location: &str) -> libloading::Result<Box<Command>> {
    let lib  = try!(Library::new(location));
    unsafe {
        let funt: Symbol<unsafe extern fn()-> Box<Command> > = lib.get(b"submit\0").unwrap();
        let command_box = funt();
        let moved = command_box.self_clone();
        Ok(moved)
    }
}

fn main() {
    println!("Hello, world!");
    //let mut command_chain: Vec<Box<Command>> = vec![];

//    let lib  = Library::new("/root/IdeaProjects/rust-dynload/command/target/debug/libembed.so").unwrap();
//        unsafe {
//            let submit: Symbol<unsafe extern fn()-> Box<Command> > =
//            lib.get(b"submit\0").unwrap();
//            let command_box = submit();
//            command_box.process(vec![]);
//
//    }

    println!("?");
    let thing = get_command("/root/IdeaProjects/rust-dynload/command/target/debug/libembed.so").unwrap();
    println!("?");
    thing.process(vec![]);
    println!("{}", thing.get_name());
    println!("?");
    println!("goodbye");

    //command_chain.push();
}