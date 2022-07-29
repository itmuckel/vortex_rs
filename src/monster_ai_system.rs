use crate::{console, FieldOfView, Monster, Name, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, FieldOfView>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, fov, monster, name) = data;

        for (fov, _monster, name) in (&fov, &monster, &name).join() {
            if fov.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}
