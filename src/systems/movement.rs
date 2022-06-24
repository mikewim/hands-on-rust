use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

use crate::camera::Camera;
use crate::components::{Player, WantsToMove};
use crate::map::Map;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    commands: &mut CommandBuffer,
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .expect("Failed to get entry ref")
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }

    commands.remove(*entity);
}
