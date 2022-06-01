use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Map(vec![vec![0; MAP_SIZE as usize]; MAP_SIZE as usize]))
        .insert_resource(RoomsData(Vec::new()))
        .add_startup_system(spawn_world)
        .add_startup_system(load_rooms.before(spawn_world));
    }
}

// Components
#[derive(Component)]
struct Wall;

// Resources
pub struct Map(pub Vec<Vec<u32>>);
struct RoomsData(Vec<Vec<String>>);

fn find_rooms(room_count: u32) -> [[i32; 20]; 20] {
    // Pick random point in rooms_grid
    // Mark it as occupied
    // Add its adjecent coords to rooms_available
    let mut rooms_grid = [[0; 20]; 20];
    let mut rooms_available = Vec::new();

    for room in 0..room_count {
        if room == 0 {
            rooms_grid[10][10] = 1;
            rooms_available.push((10, 9));
            rooms_available.push((10, 11));
            rooms_available.push((9, 10));
            rooms_available.push((11, 10));
            continue;
        }
        let rnd = rand::thread_rng().gen_range(0, rooms_available.len());
        let new_room = rooms_available[rnd];
        rooms_available.remove(rnd);
        rooms_grid[new_room.0][new_room.1] = 1;
        if !rooms_available.contains(&(new_room.0+1, new_room.1)) && rooms_grid[new_room.0+1][new_room.1] != 1 {
            rooms_available.push((new_room.0+1, new_room.1));
        }
        if !rooms_available.contains(&(new_room.0-1, new_room.1)) && rooms_grid[new_room.0-1][new_room.1] != 1 {
            rooms_available.push((new_room.0-1, new_room.1));
        }
        if !rooms_available.contains(&(new_room.0, new_room.1+1)) && rooms_grid[new_room.0][new_room.1+1] != 1 {
            rooms_available.push((new_room.0, new_room.1+1));
        }
        if !rooms_available.contains(&(new_room.0, new_room.1-1)) && rooms_grid[new_room.0][new_room.1-1] != 1 {
            rooms_available.push((new_room.0, new_room.1-1));
        }
    } 
    return rooms_grid;
}

fn load_rooms(mut rooms: ResMut<RoomsData>) {
    let file = File::open("assets/rooms.dat").unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let line: Vec<String> = line.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        rooms.0.push(line);
    }
}

fn spawn_world(mut commands: Commands, mut map: ResMut<Map>, rooms: Res<RoomsData>) {
    let rooms_grid = find_rooms(8);
    for (y, row) in rooms_grid.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if rooms_grid[y][x] == 1 {
                for y_room in 0..ROOM_SIZE as i32 {
                    for x_room in 0..ROOM_SIZE as i32 {
                        // Draw only the outter box
                        if y_room == 0 || y_room == ROOM_SIZE as i32 - 1 || x_room == 0 || x_room == ROOM_SIZE as i32 - 1 {
                            // Leave room for corridors
                            if (x_room == 4 || x_room == 5) && y_room == 0                    && rooms_grid[y - 1][x] == 1 { continue }
                            if (x_room == 4 || x_room == 5) && y_room == ROOM_SIZE as i32 - 1 && rooms_grid[y + 1][x] == 1 { continue }
                            if (y_room == 4 || y_room == 5) && x_room == 0                    && rooms_grid[y][x - 1] == 1 { continue }
                            if (y_room == 4 || y_room == 5) && x_room == ROOM_SIZE as i32 - 1 && rooms_grid[y][x + 1] == 1 { continue }

                            commands.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: WALL_COLOR,
                                    ..default()
                                },
                                transform: Transform {
                                    translation: Vec3::new((x as f32 * CELL_SIZE * ROOM_SIZE) as f32 + x_room as f32 * CELL_SIZE, (-(y as f32) * CELL_SIZE * ROOM_SIZE) as f32 + -y_room as f32 * CELL_SIZE, 0.0),
                                    scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Position((x as i32 + x_room) as u32, (y as i32 + y_room) as u32))
                            .insert(Wall);
                            map.0[(x as i32 * ROOM_SIZE as i32 + x_room) as usize][(y as i32 * ROOM_SIZE as i32 + y_room) as usize] = 1;
                        }
                    }
                }
                // Draw room data
                let room_data = rooms.0[rand::thread_rng().gen_range(0, rooms.0.len())].clone();
                println!("{:?}", room_data);
                for cell in room_data {
                    if cell.chars().nth(0).unwrap() == '1' { // It's a wall
                        let x_room = cell.chars().nth(1).unwrap() as i32 - '0' as i32;      // Hackerman
                        let y_room = cell.chars().nth(2).unwrap() as i32 - '0' as i32; 

                        commands.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: WALL_COLOR,
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new((x as f32 * CELL_SIZE * ROOM_SIZE) as f32 + (x_room + 1) as f32 * CELL_SIZE, (-(y as f32) * CELL_SIZE * ROOM_SIZE) as f32 + -((y_room + 1) as i32) as f32 * CELL_SIZE, 0.0),
                                scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Position((x as i32 + x_room + 1) as u32, (y as i32 + y_room + 1) as u32))
                        .insert(Wall);
                        map.0[(x as i32 * ROOM_SIZE as i32 + x_room + 1) as usize][(y as i32 * ROOM_SIZE as i32 + y_room + 1) as usize] = 1;
                    }
                }
            }
        }
    }
}