#![warn(clippy::pedantic)]
use super::prelude::*;

fn is_a_player_entity(ecs: &SubWorld, want_move: &WantsToMove) -> bool {
    return ecs
        .entry_ref(want_move.entity)
        .unwrap()
        .get_component::<Player>()
        .is_ok();
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.is_a_valid_movement(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }

            // Check if this entity is a player, if so, update the camera
            if is_a_player_entity(ecs, want_move) {
                camera.on_player_move(want_move.destination);
            }
        }
    }
    // Remove from the queue to avoid reprocess the same message.
    commands.remove(*entity)
}
