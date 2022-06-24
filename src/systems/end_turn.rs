use bracket_lib::prelude::Point;
use legion::world::SubWorld;
use legion::*;

use crate::components::{AmuletOfYala, Health, Player};
use crate::turn_state::TurnState;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(#[resource] turn_state: &mut TurnState, ecs: &SubWorld) {
    let mut new_state = match turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => {
            println!("Unhandled turn state!");
            TurnState::AwaitingInput
        }
    };

    if let Some(player) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
    {
        if player.0.current < 1 {
            new_state = TurnState::GameOver;
        }

        if let Some(amulet_pos) = <&Point>::query()
            .filter(component::<AmuletOfYala>())
            .iter(ecs)
            .next()
        {
            if *amulet_pos == *player.1 {
                new_state = TurnState::Victory;
            }
        }
    }

    *turn_state = new_state;
}
