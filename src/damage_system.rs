use crate::gamelog::GameLog;
use crate::{CombatStats, Name, Player, SufferDamage};
use specs::prelude::*;
use std::process::exit;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
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
            let names = ecs.read_storage::<Name>();
            let entities = ecs.entities();
            let mut log = ecs.write_resource::<GameLog>();
            for (entity, stats) in (&entities, &combat_stats).join() {
                if stats.hp < 1 {
                    let player = players.get(entity);
                    match player {
                        None => {
                            let victim_name = names.get(entity);
                            if let Some(name) = victim_name {
                                log.entries.push(format!("{} is dead", name.name));
                            }
                            dead.push(entity);
                        }
                        Some(_) => {
                            log.entries.push("You are dead!".to_string());
                            exit(0);
                        }
                    }
                }
            }
        }

        ecs.delete_entities(&dead).expect("Unable to delete");
    }
}
