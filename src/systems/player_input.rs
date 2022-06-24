use bracket_lib::prelude::{Point, VirtualKeyCode};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

use crate::components::{Enemy, Health, Player, WantsToAttack, WantsToMove};
use crate::turn_state::TurnState;

#[system]
#[write_component(TurnState)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Enemy)]
#[write_component(TurnState)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let entity_query = <(&Point, Entity)>::query;

            let player = entity_query()
                .filter(component::<Player>())
                .iter(ecs)
                .next()
                .unwrap();

            let destination = *player.0 + delta;

            let enemy = entity_query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .find(|(pos, _)| **pos == destination);

            if let Some(target) = enemy {
                commands.push((
                    (),
                    WantsToAttack {
                        victim: *target.1,
                        attacker: *player.1,
                    },
                ));
            } else {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *player.1,
                        destination,
                    },
                ));
            }
        } else {
            let mut player_health = <&mut Health>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .next()
                .unwrap();

            player_health.current = i32::min(player_health.max, player_health.current + 1)
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
