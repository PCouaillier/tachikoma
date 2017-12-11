mod command_queue;
mod planet_commander;

use hlt::game::Game;
use hlt::logging::Logger;
use strategy::commander::command_queue::CommandQueue;

#[allow(dead_code)]
pub fn run(_game: &Game, _name: &str, mut _logger: Logger) {
}


#[allow(dead_code)]
pub trait Commander {
    fn add_command(commands: &mut CommandQueue);
}
