use std::process::exit;
use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::{AccessorCow, RunningTime};
use crate::{CombatStats, Player, SufferDamage};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage)
        in (&mut stats, &damage).join() {
            stats.hp -= damage.amounts.iter().sum::<i32>();
        }

        damage.clear();
    }
}

impl DamageSystem {
    pub fn delete_the_dead(ecs: &mut World) {
        let mut dead: Vec<Entity> = Vec::new();
        {
            let combat_stats = ecs.read_storage::<CombatStats>();
            let players = ecs.read_storage::<Player>();
            let entities = ecs.entities();
            for (entity, stats) in (&entities, &combat_stats).join() {
                if stats.hp < 1 {
                    let player = players.get(entity);
                    match player {
                        None => dead.push(entity),
                        Some(_) => {
                            console::log("You are dead!");
                            exit(0);
                        }
                    }
                }
            }
        }

        ecs.delete_entities(&dead).expect("Unable to delete");
    }
}