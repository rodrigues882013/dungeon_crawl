mod camera;
mod component;
mod game_state;
mod map;
mod map_builder;
mod spawner;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::core::camera::*;
    pub use crate::core::component::*;
    pub use crate::core::game_state::*;
    pub use crate::core::map::*;
    pub use crate::core::map_builder::*;
}
