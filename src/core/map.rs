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

    fn valid_exit(&self, origin: Point, delta: Point) -> Option<usize> {
        let destination = origin + delta;

        if self.in_bounds(destination) {
            if self.is_a_valid_movement(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exists = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exist(location, Point::new(-1, 0)) {
            exists.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exist(location, Point::new(1, 0)) {
            exists.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exist(location, Point::new(0, -1)) {
            exists.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exist(location, Point::new(0, 1)) {
            exists.push((idx, 1.0))
        }

        exists
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}
