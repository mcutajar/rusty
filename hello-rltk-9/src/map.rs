use rltk::{RGB, Rltk, RandomNumberGenerator};
use super::{Rect};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls.
/// No guarantees that it won't look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn apply_room_to_map(room : &Rect, map: &mut [TileType]) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms : Vec<Rect> = Vec::new();
    const MAX_ROOMS : i32 = 30;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);
            rooms.push(new_room);
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.25, 0.25, 0.25),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.75, 0.75, 0.75),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn _apply_horizontal_tunnel(map: &mut [TileType], x1:i32, x2:i32, y:i32) {
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}