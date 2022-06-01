mod player;
mod world;
mod globals;

use crate::globals::*;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::{prelude::*, window::*};
use rand::Rng;

// Components
#[derive(Component)]
struct Position (u32, u32);

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
        .add_plugins(DefaultPlugins)
        .run();
}