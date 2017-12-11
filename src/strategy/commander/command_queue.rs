use hlt::command::Command;

pub trait CommandQueue {
    fn push_command(&mut self, command: Command);
}

impl CommandQueue for Vec<Command> {
    fn push_command(&mut self, command: Command) {
        self.push(command);
    }
}

