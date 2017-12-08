/*
 * This is a Rust implementation of the Settler starter bot for Halite-II
 * For the most part, the code is organized like the Python version, so see that
 * code for more information.
 */

mod hlt;
mod strategy;

use hlt::game::Game;
use hlt::logging::Logger;
use strategy::close_to_close::run;

fn main() {
    let name = "Tachikoma";

    // Initiailize the game
    let game = Game::new();

    // Initialize logging
    let mut logger = Logger::new(game.my_id);
    logger.log(&format!("Starting my {} bot!", name));

    //run(&game, name);
    run(&game, name, logger);
}
