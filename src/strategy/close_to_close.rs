extern crate rand;

use hlt::game::Game;
use hlt::entity::{Entity, Ship, Planet, DockingStatus};
use hlt::logging::Logger;
use rand::Rng;
use std::f64::MAX;

fn closest_planet<'a>(planets: &'a [Planet], ship: &Ship) -> &'a Planet {
    let mut d: f64 = MAX;
    let mut p: Option<&Planet> = None;
    for planet in planets {
        let d2 = planet.distance_with(ship);
        if d2 < d {
            d = d2;
            p = Some(planet);
        }
    }
    p.unwrap()
}
fn closest_planet_not_owned<'a>(planets: &'a [Planet], ship: &Ship) -> Option<&'a Planet> {
    let mut d: f64 = MAX;
    let mut p: Option<&Planet> = None;
    for planet in planets {
        if !planet.is_owned() {
            let d2 = planet.distance_with(ship);
            if d2 < d {
                d = d2;
                p = Some(planet);
            }
        }
    }
    if p.is_some() {
        return p;
    }
    p
}

fn closest_planet_not_owned_by<'a>(planets: &'a [Planet], ship: &Ship, player_id: usize) -> Option<&'a Planet> {
    let mut d: f64 = MAX;
    let mut p: Option<&Planet> = None;
    for planet in planets {
        if !planet.is_owned() {
            let d2 = planet.distance_with(ship);
            if d2 < d {
                d = d2;
                p = Some(planet);
            }
        }
    }
    if p.is_some() {
        return p;
    }

    for planet in planets {
        if planet.owner.map_or(true, |i|i != player_id as i32 ){
            let d2 = planet.distance_with(ship);
            if d2 < d {
                d = d2;
                p = Some(planet);
            }
        }
    }
    p
}

#[allow(unused)]
pub fn run(game: &Game, name: &str, mut logger: Logger)
{
    // Retrieve the first game map
    let game_map = game.update_map();

    // You can preprocess things here,
    // you have 60 seconds...

    // Once you are done, send a "ready to work"
    game.send_ready(name);
    let player_id = game_map.get_my_id();
    let mut command_queue = Vec::new();
    let mut rng = rand::thread_rng();

    loop {

        // Update the game state
        let game_map = game.update_map();

        // Loop over all of our player's ships
        for ship in game_map.me().all_ships() {
            // Ignore ships that are docked or in the process of docking
            if ship.docking_status != DockingStatus::UNDOCKED {
                continue;
            }

            if let Some(closest) = closest_planet_not_owned(game_map.all_planets(), ship) {
                if ship.can_dock(closest) {
                    command_queue.push(ship.dock(closest));
                    continue;
                }
                logger.log(&format!("distance : {}", &ship.distance_with(closest)));
                let navigate = ship.navigate(closest, &game_map, 4);

                if navigate.is_some() {
                    command_queue.push(navigate.unwrap());
                    continue;
                }
            }

            let closest = closest_planet(game_map.all_planets(), ship);
            if ship.can_dock(closest) && rng.gen::<u32>() % 5 < 4 {
                command_queue.push(ship.dock(closest));
                continue;
            }

            // get closest
            let closest = closest_planet_not_owned_by(game_map.all_planets(), ship, player_id);

            if closest.is_none() {
                logger.log("no planet found");
                continue;
            }

            logger.log("planet found");

            let closest = closest.unwrap();

            if ship.can_dock(closest) {
                command_queue.push(ship.dock(closest));
                continue;
            }

            let navigate = ship.navigate(closest, &game_map, 4);

            if navigate.is_some() {
                command_queue.push(navigate.unwrap());
            }
        }
        // Send our commands to the game
        game.send_command_queue(&command_queue);
        command_queue.clear();
    }
}
