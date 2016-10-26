pub trait Command {
    /// Process command input
    fn process(&self, args: Vec<String>);

    fn get_name(&self) -> &'static str;

    fn self_clone(&self) -> Box<Command>;
}