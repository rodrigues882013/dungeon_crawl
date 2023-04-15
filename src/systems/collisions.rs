#![warn(clippy::pedantic)]
use super::prelude::*;
use crate::core::prelude::query::{And, ComponentFilter, Passthrough, Query};

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_position = get_players(ecs);
    kill_enemies(ecs, commands, player_position);
}

fn get_players(ecs: &mut SubWorld) -> Point {
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_position = *pos);
    return player_position;
}

fn kill_enemies(ecs: &mut SubWorld, commands: &mut CommandBuffer, player_position: Point) {
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_position)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
