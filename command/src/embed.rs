extern crate shared;
use shared::Command;

use std::result;

#[no_mangle]
pub extern fn submit() -> Box<Command>{
    Box::new(ExampleCommand{name: ""})
}

pub struct ExampleCommand{
    pub name: &'static str,
}

impl Command for ExampleCommand{
    fn process(&self, args: Vec<String>){
        for str in args{
            println!("ding{}", str);
        }
    }
}

