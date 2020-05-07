use crate::rlib::object::Object;
use crate::rlib::object::Actor;
use crate::rlib::object::Ai;
use crate::rlib::combat::DeathCallback;

pub enum Mob {
    OrcGrunt,
    DragonRed,
}

pub fn make_mob(mob: Mob, x: i32, y: i32, dlevel: i32) -> Object {
    let new_mob: Object;
    match mob {
        Mob::OrcGrunt => new_mob = make_orc_grunt(x, y, dlevel),
        Mob::DragonRed => new_mob = make_dragon_red(x, y, dlevel),
    }
    return new_mob;
}

fn make_orc_grunt(x: i32, y: i32, dlevel: i32) -> Object {
    let mut orc = Object::new(x, y, dlevel, 'o', "orc", "fg_green", "nocolor", true, false, false, false);
    orc.actor = Some(Actor {
        max_hp: 3, hp: 3, ac: 0, strength: 2, dex: 1, int: 1, stealth: 10,
        mana: 0, max_mana: 0, resist_magic: 10, resist_heat: 10, resist_cold: 10, resist_shock: 10,
        speed: 10, piety: 10, target_x: None, target_y: None, mob_inv: None, alive: true, on_death: DeathCallback::Mob,
    });
    orc.ai = Some(Ai::Basic);
    return orc;
}

fn make_dragon_red(x: i32, y: i32, dlevel: i32) -> Object {
    let mut dragon = Object::new(x, y, dlevel, 'D', "dragon", "fg_red", "nocolor", true, false, false, false);
    dragon.actor = Some(Actor {
        max_hp: 3, hp: 6, ac: 0, strength: 5, dex: 1, int: 1, stealth: 10,
        mana: 20, max_mana: 20, resist_magic: 10, resist_heat: 10, resist_cold: 10, resist_shock: 10,
        speed: 10, piety: 10, target_x: None, target_y: None, mob_inv: None, alive: true, on_death: DeathCallback::Mob
    });
    dragon.ai = Some(Ai::Basic);
    return dragon;
}