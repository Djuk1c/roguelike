use bevy::{prelude::*};

pub const SCREEN_SIZE: f32 = 800.0;
pub const MAP_SIZE: u32 = 350;
pub const ROOM_SIZE: f32 = 10.0;
pub const CELL_SIZE: f32 = 20.0;
pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const PLAYER_COLOR: Color = Color::rgb(0.0, 0.85, 0.0);
pub const ENEMY_COLOR: Color = Color::rgb(0.5, 0.5, 0.0);
pub const ENEMY_ALERT_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);