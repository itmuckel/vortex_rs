use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{FieldOfView, Monster, Name, Player, Position, Renderable};
use crate::map::{Map, TileType};
use crate::monster_ai_system::MonsterAI;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

const FLOOR_COLOR: RGBA = RGBA {
    r: 0.3,
    g: 0.3,
    b: 0.3,
    a: 1.0,
};

const TRANSPARENT_COLOR: RGBA = RGBA {
    r: 0.3,
    g: 0.3,
    b: 0.3,
    a: 0.0,
};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        draw_map(&self.ecs, ctx);

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, FLOOR_COLOR, render.glyph);
            }
        }
    }
}

fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, _fov) in (&mut players, &mut fovs).join() {
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
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<FieldOfView>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // monsters
    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let roll = rng.roll_dice(1, 2);
        let name: String;
        let glyph: FontCharType;
        match roll {
            1 => {
                glyph = to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = to_cp437('o');
                name = "Orc".to_string();
            }
        };
        gs.ecs
            .create_entity()
            .with(Monster {})
            .with(Name { name: format!("{} #{}", name, i) })
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: TRANSPARENT_COLOR,
            })
            .with(FieldOfView {
                visible_tiles: vec![],
                range: 8,
                dirty: true,
            })
            .build();
    }

    // Player
    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Name { name: "Player".to_string() })
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: TRANSPARENT_COLOR,
        })
        .with(FieldOfView {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .build();


    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    main_loop(context, gs)
}
