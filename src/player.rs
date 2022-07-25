use bracket_lib::prelude::*;
use num::clamp;
use specs::prelude::*;

use crate::components::Player;
use crate::map::xy_idx;
use crate::{Position, State, TileType};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_x = clamp(pos.x + delta_x, 0, 79);
        let dest_y = clamp(pos.y + delta_y, 0, 49);
        let destination_idx = xy_idx(dest_x, dest_y);

        if map[destination_idx] != TileType::Wall {
            pos.x = dest_x;
            pos.y = dest_y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4
            => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6
            => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8
            => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2
            => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
