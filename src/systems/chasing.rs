use bracket_lib::prelude::{Algorithm2D, DijkstraMap, DistanceAlg, Point};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

use crate::components::{ChasingPlayer, Health, Player, WantsToAttack, WantsToMove};
use crate::map::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[system]
#[read_component(ChasingPlayer)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player = <(&Point, Entity)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let player_idx = map_idx(*player.0);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = map_idx(*pos);

        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player.0);
            // 1.2 here is a good number to escape float rounding error but still approximate > 1.0
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player.0
            };

            if destination == *player.0 {
                commands.push((
                    (),
                    WantsToAttack {
                        victim: *player.1,
                        attacker: *entity,
                    },
                ));
            } else {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        };
    })
}
