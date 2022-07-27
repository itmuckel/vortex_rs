use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{FieldOfView, Player, Position, Renderable};
use crate::map::{Map, TileType};
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

mod components;
mod map;
mod player;
mod rect;
mod visibility_system;

const FLOOR_COLOR: RGBA = RGBA {
    r: 0.3,
    g: 0.3,
    b: 0.3,
    a: 1.0,
};

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        draw_map(&self.ecs, ctx);

        player_input(self, ctx);

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, fov) in (&mut players, &mut fovs).join() {
        let mut x = 0;
        let mut y = 0;
        for (idx, tile) in map.tiles.iter().enumerate() {
            if map.revealed_tiles[idx] {
                let glyph;
                let mut fg;
                match tile {
                    TileType::Floor => {
                        glyph = to_cp437('█');
                        fg = FLOOR_COLOR;
                    }
                    TileType::Wall => {
                        glyph = to_cp437('█');
                        fg = RGBA::from_u8(0, 20, 70, 255);
                    }
                }
                if !map.visible_tiles[idx] {
                    fg = fg.lerp(BLACK.into(), 0.5)
                }
                ctx.set(x, y, fg, RGB::from_u8(0, 0, 0), glyph);
            }

            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("vortex").build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<FieldOfView>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    // Player
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: FLOOR_COLOR,
        })
        .with(Player {})
        .with(FieldOfView {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .build();

    main_loop(context, gs)
}
