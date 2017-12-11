use hlt::game::Game;
use hlt::logging::Logger;
use hlt::entity::{DockingStatus};

#[allow(unused)]
pub fn run(game: &Game, name: &str, mut _logger: Logger)
{
    // Retrieve the first game map
    let game_map = game.update_map();

    // You can preprocess things here,
    // you have 60 seconds...

    // Once you are done, send a "ready to work"
    game.send_ready(name);
    let mut command_queue = Vec::new();

    loop {

        // Update the game state
        let game_map = game.update_map();

        // Loop over all of our player's ships
        for ship in game_map.me().all_ships() {
            // Ignore ships that are docked or in the process of docking
            if ship.docking_status != DockingStatus::UNDOCKED {
                continue;
            }

            // Loop over all planets
            for planet in game_map.all_planets() {
                // Ignore unowned planets
                if planet.is_owned() {
                    continue;
                }

                // If we are close enough to dock, do it!
                if ship.can_dock(planet) {
                    command_queue.push(ship.dock(planet))
                } else {
                    // If not, navigate towards the planet
                    let navigate_command = ship.navigate(planet, &game_map, 90);
                    if let Some(command) = navigate_command {
                        command_queue.push(command)
                    }

                }
                break;
            }
        }
        // Send our commands to the game
        game.send_command_queue(&command_queue);
        command_queue.clear();
    }
}
