mod player;
mod world;

use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::{prelude::*, window::*};
use rand::Rng;

pub const SCREEN_SIZE: f32 = 800.0;
pub const MAP_SIZE: u32 = 350;
pub const ROOM_SIZE: f32 = 10.0;
pub const CELL_SIZE: f32 = 20.0;
pub const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const PLAYER_COLOR: Color = Color::rgb(0.9, 0.0, 0.0);
pub const BAT_COLOR: Color = Color::rgb(0.0, 0.6, 0.0);
pub const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

// Components
#[derive(Component)]
struct Position (u32, u32);

#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Bat;

// Resources
struct MoveTimer(Timer);        // Timer that player has to wait to move again
struct MoveEnemies(bool);       // Bool that updates after MoveTimer finishes, to move enemies
struct Map(Vec<Vec<u32>>);

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
        .insert_resource(Map(vec![vec![0; MAP_SIZE as usize]; MAP_SIZE as usize]))
        .insert_resource(MoveTimer(Timer::from_seconds(0.135, false)))
        .insert_resource(MoveEnemies(false))
        .add_plugin(WorldPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

//fn spawn_enemies(mut commands: Commands) {
//    for _i in 0..3 {
//        let x: u32 = rand::thread_rng().gen_range(2, MAP_SIZE-2);
//        let y: u32 = rand::thread_rng().gen_range(2, MAP_SIZE-2);
//        commands.spawn_bundle(SpriteBundle {
//            sprite: Sprite {
//                color: BAT_COLOR,
//                ..default()
//            },
//            transform: Transform {
//                translation: Vec3::new(x as f32 * CELL_SIZE, -(y as i32) as f32 * CELL_SIZE, 0.0),
//                scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
//                ..default()
//            },
//            ..default()
//        })
//        .insert(Position(x, y))
//        .insert(Enemy)
//        .insert(Bat);
//    }
//}
//
//fn move_enemies(mut enemies: Query<&mut Transform, With <Enemy>>, mut to_move: ResMut<MoveEnemies>) {
//    if to_move.0 {
//        for mut enemy in enemies.iter_mut() {
//            enemy.translation.x += CELL_SIZE;
//        }
//    }
//    to_move.0 = false;
//}