#![warn(clippy::pedantic)]
use super::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}


pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn is_in_bound(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn is_a_valid_movement(&self, point: Point) -> bool {
        let tile = self.tiles[map_idx(point.x, point.y)];
        self.is_in_bound(point) && tile == TileType::Floor
    }

    pub fn try_move(&self, point: Point) -> Option<usize> {
        if !self.is_in_bound(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
