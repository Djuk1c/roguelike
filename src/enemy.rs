use crate::globals::*;
use crate::Position;
use crate::world::Map;
use crate::line_of_sight::*;
use crate::player::{Player, MoveEnemies};
use bevy::{prelude::*};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(enemy_check_los);
    }
}

#[derive(Component)]
pub struct Enemy;

fn enemy_check_los(player: Query<&mut Position, With<Player>>, mut enemies: Query<(&mut Position, &mut Sprite), (With<Enemy>, Without<Player>)>, map: Res<Map>, mut to_move: ResMut<MoveEnemies>) {
    let view_distance = 6;
    if !to_move.0 {return}      // Player hasnt made its turn
    let player = player.single();
    for mut enemy in enemies.iter_mut() {
        let mut wall = false;
        // View distance
        if (enemy.0.0 as i32 - player.0 as i32).abs() > view_distance || (enemy.0.1 as i32 - player.1 as i32).abs() > view_distance {
            enemy.1.color = ENEMY_COLOR;
            continue
        }
        let points = get_line(Point2d::new(enemy.0.0, enemy.0.1), Point2d::new(player.0, player.1));
        for point in points {
            if map.0[point.x as usize][point.y as usize] == 1 {
                wall = true;
                break;
            }
        }
        if !wall {
            enemy.1.color = ENEMY_ALERT_COLOR;
        }
    }
    to_move.0 = false;
}