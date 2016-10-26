extern crate shared;
use shared::Command;

use std::result;

#[no_mangle]
pub extern fn submit() -> Box<Command>{
    Box::new(ExampleCommand{
        //name: "Test name"
    })
}

pub struct ExampleCommand{
    //pub name: &'static str,
}

impl Command for ExampleCommand{
    fn process(&self, args: Vec<String>){
        println!("Hey look a thing");
        for str in args{
            println!("ding{}", str);
        }
    }

    fn get_name(&self) -> &'static str{
        "thingey"
    }

    fn self_clone(&self) -> Box<Command>{
        //let new_name = self.name;
          Box::new(ExampleCommand{
                   //name: new_name
          })
    }
}

