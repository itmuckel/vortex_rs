mod map;
mod player;
mod components;
mod rect;

use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{LeftMover, Player, Position, Renderable};
use crate::map::{new_map_rooms_and_corridors, new_map_test, TileType};
use crate::player::player_input;


struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut left_walker = LeftWalker {};
        left_walker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        {
            let map = self.ecs.fetch::<Vec<TileType>>();
            draw_map(&map, ctx);
        }

        player_input(self, ctx);

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(127, 127, 127), RGB::from_u8(0, 0, 0), to_cp437(' '))
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(0, 127, 0), RGB::from_u8(0, 0, 0), to_cp437('█'))
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("vortex")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    main_loop(context, gs)
}
