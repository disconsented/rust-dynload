pub trait Command {
    /// Process command input
    fn process(&self, args: Vec<String>);
}