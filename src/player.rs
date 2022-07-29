use bracket_lib::prelude::*;
use num::clamp;
use specs::prelude::*;

use crate::components::Player;
use crate::{CombatStats, FieldOfView, Map, Position, RunState, State, TileType, WantsToMelee};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let mut player_pos = ecs.write_resource::<Point>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, fov) in (&entities, &mut players, &mut positions, &mut fovs).join() {
        let dest_x = clamp(pos.x + delta_x, 0, 79);
        let dest_y = clamp(pos.y + delta_y, 0, 49);
        let destination_idx = map.xy_idx(dest_x, dest_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(_target) => {
                    // Attack it
                    console::log("From Hell's Heart, I stab thee!");
                    wants_to_melee
                        .insert(entity, WantsToMelee { target: *potential_target })
                        .expect("Add target failed");
                    return;
                }
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = dest_x;
            pos.y = dest_y;
            fov.dirty = true;

            player_pos.x = pos.x;
            player_pos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput; } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H =>
                try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L =>
                try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K =>
                try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J =>
                try_move_player(0, 1, &mut gs.ecs),

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y =>
                try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U =>
                try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N =>
                try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B =>
                try_move_player(-1, 1, &mut gs.ecs),

            _ => { return RunState::AwaitingInput; }
        },
    }

    RunState::PlayerTurn
}
