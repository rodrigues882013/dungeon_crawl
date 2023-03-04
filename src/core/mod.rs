mod camera;
mod game_state;
mod map;
mod map_builder;
mod player;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use super::camera::*;
    pub use super::game_state::*;
    pub use super::map::*;
    pub use super::map_builder::*;
    pub use super::player::*;
}
