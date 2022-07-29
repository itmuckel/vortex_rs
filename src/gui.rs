use crate::gamelog::GameLog;
use crate::map::{HEIGHT, WIDTH};
use crate::{CombatStats, Map, Name, Player, Position};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(
        0,
        HEIGHT,
        WIDTH - 1,
        6,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(17, HEIGHT, RGB::named(YELLOW), RGB::named(BLACK), &health);
        ctx.draw_bar_horizontal(
            28,
            HEIGHT,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(RED),
            RGB::named(BLACK),
        );
    }

    let log = ecs.fetch::<GameLog>();
    let mut y = HEIGHT + 1;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    let (mouse_x, mouse_y) = ctx.mouse_pos();
    ctx.set_bg(mouse_x, mouse_y, RGB::named(GREEN3));
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let (mouse_x, mouse_y) = ctx.mouse_pos();
    if mouse_x >= map.width || mouse_y >= map.height {
        return;
    }

    let mut tooltip: Vec<String> = vec![];
    for (name, position) in (&names, &positions).join() {
        if position.x == mouse_x && position.y == mouse_y {
            let idx = map.xy_idx(position.x, position.y);
            if map.visible_tiles[idx] {
                tooltip.push(name.name.clone());
            }
        }
    }

    if tooltip.is_empty() {
        return;
    }

    let mut width: i32 = 0;
    for s in tooltip.iter() {
        if width < s.len() as i32 {
            width = s.len() as i32
        }
    }
    width += 3;

    let fg = RGB::named(WHITE);
    let bg = RGB::from_u8(100, 100, 100);

    if mouse_x > 40 {
        let arrow_pos = Point::new(mouse_x - 2, mouse_y);
        let left_x = mouse_x - width;
        let mut y = mouse_y;
        for s in tooltip.iter() {
            ctx.print_color(left_x, y, fg, bg, s);
            let padding = (width - s.len() as i32) - 1;
            for i in 0..padding {
                ctx.print_color(arrow_pos.x - i, y, fg, bg, " ");
            }
            y += 1;
            ctx.print_color(arrow_pos.x, arrow_pos.y, fg, bg, "->");
        }
    } else {
        let arrow_pos = Point::new(mouse_x + 1, mouse_y);
        let left_x = mouse_x + 3;
        let mut y = mouse_y;
        for s in tooltip.iter() {
            ctx.print_color(left_x + 1, y, fg, bg, s);
            let padding = (width - s.len() as i32) - 1;
            for i in 0..padding {
                ctx.print_color(arrow_pos.x + 1 + i, y, fg, bg, " ");
            }
            y += 1;
        }
        ctx.print_color(arrow_pos.x, arrow_pos.y, fg, bg, "<-");
    }
}
