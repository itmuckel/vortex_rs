use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::colors::{FLOOR_COLOR, TRANSPARENT_COLOR};
use crate::components::{
    BlocksTile, CombatStats, FieldOfView, Monster, Name, Player, Position, Renderable,
    SufferDamage, WantsToMelee,
};
use crate::damage_system::DamageSystem;
use crate::map::{Map, TileType};
use crate::map_indexing_system::MapIndexingSystem;
use crate::melee_combat_system::MeleeCombatSystem;
use crate::monster_ai_system::MonsterAI;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

mod colors;
mod components;
mod damage_system;
mod gamelog;
mod gui;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage_system = DamageSystem {};
        damage_system.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let mut new_run_state;
        {
            new_run_state = *self.ecs.fetch::<RunState>();
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_run_state;
        }

        DamageSystem::delete_the_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, FLOOR_COLOR, render.glyph);
            }
        }

        gui::draw_ui(&self.ecs, ctx);
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
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<FieldOfView>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

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
            .with(Name {
                name: format!("{} #{}", name, i),
            })
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
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
            .with(BlocksTile {})
            .build();
    }

    // Player
    let player_entity = gs
        .ecs
        .create_entity()
        .with(Player {})
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
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

    gs.ecs.insert(player_entity);
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to vortex!".to_string()],
    });

    main_loop(context, gs)
}
