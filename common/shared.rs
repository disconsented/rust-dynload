pub trait Process{
    fn process(&self, args: [String]);
}

pub struct Command {
    name: String
}