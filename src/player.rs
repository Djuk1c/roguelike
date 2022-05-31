use crate::*;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement.before(camera_follow))
        .add_system(camera_follow);
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Camera;

fn spawn_player(mut commands: Commands) {
    let x = (ROOM_SIZE * 10.0 + ROOM_SIZE / 2.0) as u32;
    let y = (ROOM_SIZE * 10.0 + ROOM_SIZE / 2.0) as u32;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: PLAYER_COLOR,
            ..default()
        },
        transform: Transform {
            // We know that room_grid[10][10] is always first room, always filled, so we spawn there
            translation: Vec3::new(ROOM_SIZE * 10.0 * CELL_SIZE + (ROOM_SIZE * CELL_SIZE) / 2.0, -ROOM_SIZE * 10.0 * CELL_SIZE - (ROOM_SIZE * CELL_SIZE) / 2.0, 10.0),
            scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
            ..default()
        },
        ..default()
    })
    .insert(Position(x, y))
    .insert(Player);
}

fn player_movement(mut player: Query<&mut Transform, With<Player>>, input: Res<Input<KeyCode>>, time: Res<Time>, mut timer: ResMut<MoveTimer>, mut to_move: ResMut<MoveEnemies>, map: Res<Map>, mut pos: Query<&mut Position, With<Player>>) {
    timer.0.tick(time.delta());
    let mut moved = false;
    let mut player = player.single_mut();
    let mut pos = pos.single_mut();

    if timer.0.finished() {
        if input.pressed(KeyCode::W) && map.0[pos.0 as usize][(pos.1-1) as usize] != 1 {   //x+y*MAP_SIZE
            pos.1 -= 1;
            player.translation.y += player.scale.x;
            moved = true;
        }
        else if input.pressed(KeyCode::S) && map.0[pos.0 as usize][(pos.1+1) as usize] != 1 {
            pos.1 += 1;
            player.translation.y -= player.scale.x;
            moved = true;
        }
        else if input.pressed(KeyCode::D) && map.0[(pos.0+1) as usize][pos.1 as usize] != 1 {
            pos.0 += 1;
            player.translation.x += player.scale.x;
            moved = true;
        }
        else if input.pressed(KeyCode::A) && map.0[(pos.0-1) as usize][pos.1 as usize] != 1 {
            pos.0 -= 1;
            player.translation.x -= player.scale.x;
            moved = true;
        }
        if moved {
            timer.0.reset();
            to_move.0 = true;
            println!("x: {} y: {}", pos.0, pos.1);
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    //camera.transform.translation.x = SCREEN_SIZE/2.0-CELL_SIZE/2.0;
    //camera.transform.translation.y = -(SCREEN_SIZE/2.0-CELL_SIZE/2.0);
    commands.spawn_bundle(camera)
        .insert(Camera);
}

fn camera_follow(player: Query<&Transform, With<Player>>, mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>) {
    let player = player.single();
    let mut camera = camera.single_mut();
    camera.translation = player.translation;
}