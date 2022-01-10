use rand::Rng;
use crate::rlib::object::Object;
//use crate::Actor;
//use crate::Ai;
//use crate::rlib::combat::DeathCallback;
use crate::rlib::dungen::Rectangle;
use crate::rlib::mobs::make_mob;
use crate::rlib::mobs::Mob;

const MAX_ROOM_MOBS: i32 = 4;
//const MAX_ROOM_MOBS: i32 = 1;


pub fn place_mobs(dlevel: i32, room: &Rectangle) -> Vec<Object> {
    // choose random number of monsters
    let num_mobs = rand::thread_rng().gen_range(0..MAX_ROOM_MOBS + 1);
    let mut mobs = vec![];
    for _ in 0..num_mobs {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

        let mob = if rand::random::<f32>() < 0.8 {  // 80% chance of getting an orc
            let orc = make_mob(Mob::OrcGrunt, x, y, dlevel); orc
        } else {
            let dragon = make_mob(Mob::DragonRed, x, y, dlevel); dragon
        };
        mobs.push(mob);
    }
    mobs
}
