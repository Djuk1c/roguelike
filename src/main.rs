mod player;
mod enemy;
mod world;
mod globals;
mod line_of_sight;

use crate::globals::*;
use crate::world::WorldPlugin;
use crate::player::PlayerPlugin;
use crate::enemy::EnemyPlugin;
use bevy::{prelude::*, window::*};
use rand::Rng;

// Components
#[derive(Component)]
pub struct Position (u32, u32);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: SCREEN_SIZE,
            height: SCREEN_SIZE,
            title: "Poggies".to_string(),
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugin(WorldPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}