#![warn(clippy::pedantic)]

use super::prelude::*;

fn get_enemy(enemy_type: &str) -> (i32, String, FontCharType) {
    return match enemy_type {
        "goblin" => (1, "Goblin".to_string(), to_cp437('g')),
        "orc" => (2, "Orc".to_string(), to_cp437('o')),
        "ogre" => (3, "Ogre".to_string(), to_cp437('O')),
        "ettin" => (4, "Ettin".to_string(), to_cp437('E')),
        _ => panic!("No enemy type found"),
    };
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 20,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1 => get_enemy("goblin"),
        2 => get_enemy("orc"),
        3..=8 => get_enemy("ettin"),
        _ => get_enemy("ogre"),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
