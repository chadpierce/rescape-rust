use std::cmp;
use rand::Rng;
use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
use crate::HERO;
use crate::rlib::path::bfs_test;

//#[derive(Clone, Copy, Debug, PartialEq)]
// pub enum AttackType {
//     MeleeStr,
//     MeleeDex
//     RangedStr,
//     MeleeDex,
//     Zap,

// }
 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Hero,
    Mob,
}
impl DeathCallback {
    pub fn callback(self, object: &mut Object, messages: &mut Messages) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Messages) = match self {
            Hero => hero_death,
            Mob => mob_death,
        };
        callback(object, messages);
    }
}

pub fn melee_attack(source: &mut Object, target: &mut Object) -> i32 {

    //println!("source name: {:?}", source.name);
    //println!("target name: {:?}", target.name);

    let source_rng = rand::thread_rng().gen_range(0..3);
    let target_rng = rand::thread_rng().gen_range(0..3);
    let damage = source.actor.map_or(0, |f| f.strength + source_rng) - target.actor.map_or(0, |f| f.strength + target_rng);

    //let dmg = 0;
    return damage;

}


pub fn hero_death(hero: &mut Object, messages: &mut Messages) {
    // game over
    //TODO add messages here and mob death
    messages.add("You died!", "RED");
    hero.glyph = '%';
    hero.fg_color = "fg_red".to_string();
}

pub fn mob_death(mob: &mut Object, messages: &mut Messages) {
    mob.glyph = '%';
    mob.blocks = false;
    mob.actor = None;
    mob.ai = None;
    messages.add(format!("The {} died!", mob.name), "RED".to_string());
    mob.name = format!("remains of {}", mob.name);
}

pub fn ai_take_turn(mob_id: usize, mut messages: &mut Messages, objects: &mut [Object]) {
    // basic ai turn
    //if objects[mob_id].distance_to_target() >= 2.0 {
    if objects[mob_id].distance_to(&objects[HERO]) >= 2.0 {
        // move towards player if far away
        // TODO this  should look at last known loc if target is not visible
        //let (target_x, target_y) = objects[mob_id].target_pos();
        let (target_x, target_y) = objects[0].pos();
        //move_towards(mob_id, target_x, target_y, objects, &mut messages);
        bfs_test(mob_id, objects[mob_id].x as usize, objects[mob_id].y as usize, target_x as usize, target_y as usize, objects, &mut messages);
        //dumb_chase(mob_id, target_x, target_y, objects, &mut messages);
    } else if objects[HERO].actor.map_or(false, |f| f.hp > 0) {
        // close enough to attack
        let (mob, hero) = mut_two(mob_id, HERO, objects);
        mob.attack(&mut messages, hero);
    }
    /* original
    let (mob_x, mob_y) = objects[mob_id].pos();
        if objects[mob_id].distance_to(&objects[HERO]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[HERO].pos();
            move_towards(mob_id, player_x, player_y, objects, &mut messages);
        } else if objects[HERO].actor.map_or(false, |f| f.hp > 0) {
            // close enough to attack
            let (mob, hero) = mut_two(mob_id, HERO, objects);
            mob.attack(&mut messages, hero);
        }
    */
}

/// Mutably borrow two *separate* elements from the given slice.
/// Panics when the indexes are equal or out of bounds.
pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}
