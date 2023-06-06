#![warn(clippy::pedantic)]
use super::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(fov_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(fov_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(chasing_system())
        .flush()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(fov_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}
