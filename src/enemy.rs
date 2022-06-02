use crate::globals::*;
use crate::Position;
use crate::world::Map;
use crate::line_of_sight::*;
use crate::player::{Player, MoveEnemies};
use bevy::{prelude::*};
use queues::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(enemy_move);
    }
}

#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Alerted(pub bool);


pub fn enemy_move(player: Query<&mut Position, With<Player>>, mut enemies: Query<(&mut Position, &mut Sprite, &mut Transform, &mut Alerted), (With<Enemy>, Without<Player>)>, map: Res<Map>, mut to_move: ResMut<MoveEnemies>) {
    let view_distance = 6;
    // Player hasnt made its turn
    if !to_move.0 {return}
    let player = player.single();
    for mut enemy in enemies.iter_mut() {
        let mut wall = false;
        // Checking if player is in enemy line of sight
        if (enemy.0.0 as i32 - player.0 as i32).abs() > view_distance || (enemy.0.1 as i32 - player.1 as i32).abs() > view_distance {
            if enemy.3.0 == false { continue }
        }
        let points = get_line(Point2d::new(enemy.0.0, enemy.0.1), Point2d::new(player.0, player.1));
        for point in &points {
            if map.0[point.x as usize][point.y as usize] == 1 {
                wall = true;
                break;
            }
        }
        if !wall || enemy.3.0 {
            enemy.3.0 = true;
            enemy.1.color = ENEMY_ALERT_COLOR;

            // Path finding & move to player
            let mut visited = [[false; MAP_SIZE as usize]; MAP_SIZE as usize];
            let mut queue: Queue<(u32, u32)> = queue![];
            let mut prev = [[(0, 0); MAP_SIZE as usize]; MAP_SIZE as usize];
            let d_row = [ -1, 0, 1, 0 ];
            let d_col = [ 0, 1, 0, -1 ];
            
            visited[enemy.0.0 as usize][enemy.0.1 as usize] = true;
            queue.add((enemy.0.0, enemy.0.1)).unwrap();

            'outer: while queue.size() != 0 {
                let front = queue.remove().unwrap();
                let (x, y) = front;
                for i in 0..4 {
                    let adjx = x as i32 + d_row[i];
                    let adjy = y as i32 + d_col[i];
                    if map.0[adjx as usize][adjy as usize] == 0 && !visited[adjx as usize][adjy as usize] {
                        queue.add((adjx as u32, adjy as u32)).unwrap();
                        visited[adjx as usize][adjy as usize] = true;
                        prev[adjx as usize][adjy as usize] = front;
                    }
                    else if map.0[adjx as usize][adjy as usize] == 9 {
                        prev[player.0 as usize][player.1 as usize] = front;
                        break 'outer
                    }
                }
            }

            // Reverse the path
            let (mut new_x ,mut new_y) = prev[player.0 as usize][player.1 as usize];
            while (enemy.0.0, enemy.0.1) != (prev[new_x as usize][new_y as usize]) {
                (new_x, new_y) = prev[new_x as usize][new_y as usize];
                if (new_x, new_y) == (0,0) {break}
            }

            enemy.2.translation.x = new_x as f32 * CELL_SIZE;
            enemy.2.translation.y = -(new_y as f32 * CELL_SIZE);
            enemy.0.0 = new_x;
            enemy.0.1 = new_y;
        }
    }
    to_move.0 = false;
}