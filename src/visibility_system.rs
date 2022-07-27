use super::{FieldOfView, Position};
use crate::{field_of_view, Map, Player, Point};
use specs::prelude::*;

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut fov, pos, player) = data;

        for (ent, fov, pos) in (&entities, &mut fov, &pos).join() {
            if !fov.dirty {
                continue;
            }

            fov.visible_tiles.clear();
            fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
            fov.visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            fov.dirty = false;

            let p = player.get(ent);
            if let Some(_p) = p {
                for t in &mut map.visible_tiles {
                    *t = false
                }
                for visible in &fov.visible_tiles {
                    let idx = map.xy_idx(visible.x, visible.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }
}
