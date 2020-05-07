mod rlib;
use crate::rlib::object::Object;
use crate::rlib::object::Actor;
use crate::rlib::msg::Messages;

//dimension of map
const MAP_WIDTH: i32 = 60;
const MAP_HEIGHT: i32 = 20;
// hero is always first object
const HERO: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    Turn,
    NoTurn,
    Exit,
}

fn hero_run_attack(dx: i32, dy: i32, mut objects: &mut [Object], mut messages: &mut Messages) {
    let mut hero_dx = objects[HERO].x + dx;
    let mut hero_dy = objects[HERO].y + dy;
    
    while !is_blocked(hero_dx, hero_dy, objects) {
        hero_move_attack(dx, dy, objects, messages);
        // FIXME TODO this is bad, fix with ai take turn code/func that needs to be built
        for id in 0..objects.len() { //TODO FIXME this is garbage - fix it!
            if objects[id].ai.is_some() && objects[id].dlevel == objects[HERO].dlevel {
//use termion::input::TermRead;
                rlib::combat::ai_take_turn(id, &mut messages, &mut objects);
            }
        }

        hero_dx += dx;
        hero_dy += dy;
    }
        messages.add(format!("Something stops you."), "TESTCOLOR".to_string());
}

fn hero_move_attack(dx: i32, dy: i32, objects: &mut [Object], mut messages: &mut Messages) {
    // location the hero is moving to OR attacking
    let x = objects[HERO].x + dx;
    let y = objects[HERO].y + dy;
    let z = objects[HERO].dlevel;

    // see if object is attackable
    // try to find an attackable object there
    let target_id = objects
        .iter()
        .position(|object| object.dlevel == z && object.actor.is_some() && object.pos() == (x, y));
    // attack or move
    match target_id {
        Some(0) => { move_by(HERO, dx, dy, objects, &mut messages); } //dont move into self
        Some(target_id) => {
            //messages.add("you hit someting", "color");
            let (hero, target) = rlib::combat::mut_two(HERO, target_id, objects);
            hero.attack(&mut messages, target);
        }
        None => {
            //passes here so other objects can use this func to move/attack
            move_by(HERO, dx, dy, objects, &mut messages);
        }
    }
}

// move by delta x and delta y if no object is blocking
fn move_by(id: usize, dx: i32, dy: i32, objects: &mut [Object], mut messages: &mut Messages) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, objects) {
        objects[id].set_pos(x + dx, y + dy);
        if id == HERO { rlib::cmd::list_obj_at_loc(x + dx, y + dy, &objects, &mut messages); }  //if HERO then list items on ground
    }
}
fn is_blocked(x: i32, y: i32, objects: &[Object]) -> bool {
    //check for blocking objects
    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y) && object.dlevel == objects[0].dlevel)
}

fn main() {
    let mut hero = Object::new(1, 1, 1, '@', 
        "hero", "fg_black", "bg_cyan", true, false, true, true);
    hero.actor = Some(Actor {
        max_hp: 20,
        hp: 20,
        ac: 1,
        strength: 3,
        dex: 10,
        int: 10,
        stealth: 10,
        mana: 10,
        max_mana: 10,
        resist_magic: 10,
        resist_heat: 10,
        resist_cold: 10,
        resist_shock: 10,
        speed: 10,
        piety: 10,
        target_x: None, 
        target_y: None,
        mob_inv: None,
        alive: true,
        // TODO add dest pos for mob pathfinding and retreating?
        on_death: rlib::combat::DeathCallback::Hero,
    });
    let mut objects = vec![hero];
    let mut messages = Messages::new();
    
    let test_floor = rlib::dungen::make_map(1, &mut objects);  //make d1
    objects.extend(test_floor);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::WeaponDagger, objects[HERO].x + 1, objects[HERO].y + 0);
    objects.push(new_item);

    let mut new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::WeaponAxeBroad, objects[HERO].x - 1, objects[HERO].y + 0);
    new_item.name = "Broad Axe of Chopping".to_string(); //customize item after generation
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::PotHeal, objects[HERO].x + 0, objects[HERO].y + 1);
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::RingStrength, objects[HERO].x + 1, objects[HERO].y + 1);
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::WandZap, objects[HERO].x -1, objects[HERO].y - 1);
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::AmuletYendor, objects[HERO].x +1, objects[HERO].y - 1);
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::ScrollMagicMap, objects[HERO].x -1, objects[HERO].y + 1);
    objects.push(new_item);

    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::BookCantrips, objects[HERO].x -2, objects[HERO].y -2);
    objects.push(new_item);
    
    /*//SPEED TEST
    use std::time::{Duration, Instant};
    *///END SPEED TEST

    //game loop
    loop {

        /*//SPEED TEST
        //let start = Instant::now();
        *///END SPEED TEST

        //TODO this will need to stop player from leaving bounds    
        //check map bounds
        if objects[HERO].x < 0 || objects[HERO].x > MAP_WIDTH - 1 || 
          objects[HERO].y < 0 || objects[HERO].y > MAP_HEIGHT - 1 {
            messages.add("ERROR: out of bounds!", "TEMPCOLOR");
        }
        rlib::fov::calc_fov(&mut objects);
        let map = rlib::output::build_map(&objects);
        //TODO combine build and draw map
        rlib::output::draw_map(&map, &messages, &objects);
        let player_action = rlib::input::keypress(&mut objects, &mut messages);
        if player_action == PlayerAction::Exit {
            println!("exit!");
            break;
        }

        // mobs take action

        //TODO ai_take_turn should be what is called here...calc what objects are within taht func - clean this up

        if objects[HERO].actor.unwrap().alive && player_action != PlayerAction::NoTurn {
            for id in 0..objects.len() { //TODO FIXME this is garbage - fix it!
                if objects[id].ai.is_some() && objects[id].dlevel == objects[HERO].dlevel && objects[id].actor.unwrap().target_x.is_some(){
                    rlib::combat::ai_take_turn(id, &mut messages, &mut objects);
                }
            }
        }
    /*//SPEED TEST
        let duration = start.elapsed();
        use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use std::io::{stdout, stdin, Write};
    use termion::clear;
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}",
        termion::cursor::Goto(1, 2),
        termion::cursor::Hide).unwrap();
        println!("loop time is: {:?}", duration);
        stdout.flush().unwrap();
    *///SPEED TEST END
    }
}

// fn exit_game() {
//     //for testing
//     panic!("exiting!");
// }
