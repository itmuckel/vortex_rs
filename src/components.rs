use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGBA,
}

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

/// Blocks Movement
#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amounts: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amounts.push(amount);
        } else {
            let dmg = SufferDamage {
                amounts: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}
