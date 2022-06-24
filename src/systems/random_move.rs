use bracket_lib::prelude::{Point, RandomNumberGenerator};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

use crate::components::{Health, MovingRandomly, Player, WantsToAttack, WantsToMove};

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player: (&Point, &Health, &Entity) = <(&Point, &Health, Entity)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    <(&Point, Entity)>::query()
        .filter(component::<MovingRandomly>())
        .iter(ecs)
        .for_each(|(pos, entity)| {
            let mut rng = RandomNumberGenerator::new();

            let random_direction = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let destination = random_direction + *pos;

            if destination == *player.0 {
                commands.push((
                    (),
                    WantsToAttack {
                        victim: *player.2,
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
        });
}
