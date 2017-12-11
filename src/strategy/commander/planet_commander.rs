use hlt::entity::{ DockingStatus, Entity, Planet, Ship};
use ::strategy::commander::Commander;
use ::strategy::commander::command_queue::CommandQueue;

#[allow(dead_code)]
pub struct PlanetCommander<'a> {
    planet: &'a Planet,
    docked_ships: Vec<&'a Ship>,
    defense_ships: Vec<&'a Ship>,
}

#[allow(dead_code)]
impl <'a> PlanetCommander<'a> {

    #[allow(dead_code)]
    fn new(planet: &'a Planet, ships: Vec<&'a Ship>) -> PlanetCommander<'a> {
        let mut docked_ships = Vec::<&'a Ship>::new();
        let mut defense_ships = Vec::<&'a Ship>::new();

        for ship in ships {
            if ship.docking_status == DockingStatus::UNDOCKED {
                defense_ships.push(ship);
            } else {
                docked_ships.push(ship);
            }
        }

        PlanetCommander { planet, docked_ships, defense_ships }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn close_to_planet<'b>(planet: &Planet, ships: &[&'b Ship]) -> Vec<&'b Ship> {
        let mut close = Vec::<&Ship>::new();
        for ship in ships {
            if (*ship).distance_with(&(*planet).closest_point_to(*ship, 1f64)) < 6f64 {
                close.push(*ship);
            }
        }
        close
    }
}

impl <'a> Commander for PlanetCommander<'a> {
    #[allow(unused_variables)]
    fn add_command(commands: &mut CommandQueue) {
    }
}