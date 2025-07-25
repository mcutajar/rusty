use super::{Map, Player, Position, RunState, State, TileType, Viewshed};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();
    let mut ppos = ecs.write_resource::<Point>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        // Nothing happened
        None => return RunState::Paused,
        Some(key) => match key {
            // ←
            VirtualKeyCode::Left
            | VirtualKeyCode::Numpad4
            | VirtualKeyCode::A
            | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            // →
            VirtualKeyCode::Right
            | VirtualKeyCode::Numpad6
            | VirtualKeyCode::D
            | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            // ↑
            VirtualKeyCode::Up
            | VirtualKeyCode::Numpad8
            | VirtualKeyCode::W
            | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            // ↓
            VirtualKeyCode::Down
            | VirtualKeyCode::Numpad2
            | VirtualKeyCode::S
            | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // ↙
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),
            // ↖
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut gs.ecs),
            // ↘
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut gs.ecs),
            // ↗
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
