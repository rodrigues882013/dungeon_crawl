#![warn(clippy::pedantic)]
use super::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(collisions_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}
