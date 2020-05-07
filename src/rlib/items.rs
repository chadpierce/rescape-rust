use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
use crate::HERO;
use crate::rlib;


// TODO make item "effects" and "stats" 
// each of these are attributes that are looped through for each item when it is used
// this would make things more modular and reduce repetitive code

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    pub id: ItemID,
    pub held: bool,
    pub slot: char,

    // TODO decide on u8 or i32? speed vs memory?
    // TODO should Item be broken down into wearables and consumables (ItemClass)- or more global
    // wearable, consumable, weapon

    pub level: u8, //for armor enchant, wand level, ring ecnhant, etc
    pub charges: u8, //TODO make optional?
    pub buc_status: BUC,
    //pub item_class: 
    pub item_type: Option<Type>,
    pub body_slot: Option<BodySlot>,
    pub damage: u8,
    pub attack_speed: u8,
    pub ac: u8,
    pub brand: u8,
    pub stack: u8,
    //pub effects: Option<Effects>,
    //pub stats: Option<Stats>,
        
}

impl Item {
    fn get_stats(&self) -> (u8, u8, BUC) {
        let level = self.level;
        let charges = self.charges;
        let buc_status = self.buc_status;
        return (level, charges, buc_status);
    }
    // fn set_level(&mut self, new_level: u8) {
    //     self.level = new_level;
    // }
    // fn set_charges(&mut self, new_charges: u8) {
    //     self.charges = new_charges;
    // }
    // fn set_buc_status(&mut self, new_buc: BUC) {
    //     self.buc_status = new_buc;
    // }
    // fn set_held(&mut self, is_held: bool) {
    //     self.held = is_held;
    // }
    pub fn is_in_use(&mut self) -> bool {
        if self.body_slot.is_some() { return true; }
        else { return false; }
    }
}

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum Effects {
//     Damage,
//     AttackSpeed,
//     Ac,
//     Brand,
// }

// pub enum Stats {
//     Health,
//     MaxHealth,
//     Strength,
//     Dex,
//     Int,
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUC {
    _Normal,
    _Blessed,
    _Cursed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type { 
    //Helm,
    Amulet,
    //Armor,
    //Gloves,
    //Boots,
    Ring,
    Weapon1h,
    //Weapon2h,
    //Shield,
    //Buckler,
    Potion,
    Scroll,
    Book,
    Wand,
    //Ammo,

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BodySlot {
    //Head,
    //Neck,
    //Chest,
    //Hands,
    //Feet,
    //RightFinger,
    LeftFinger,
    RightWeapon, //TODO change to WeaponOne/WeaponTwo
    //LeftWeapon,
    //Buckler,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemID {
    PotHeal,
    ScrollMagicMap,
    //BookBlink,
    RingStrength,
    //RingDex,
    AmuletYendor,
    WandZap,
    //ArmorLeather,
    //CloakElven,
    //RobePlain,
    //HelmLeather,
    //GlovesLeather,
    //ShieldBuckler,
    //BootsLow,
    WeaponDagger,
    WeaponAxeBroad,
    //WeaponBroadSword,
    //WeaponBowElven,
    BookCantrips,
    //MiscStone,
}

pub fn make_item(item_id: ItemID, x: i32, y: i32) -> Object {
    let new_item: Object;
    match item_id {
        ItemID::WeaponDagger => new_item = make_weapon_dagger(x, y),
        ItemID::WeaponAxeBroad => new_item = make_weapon_axe_broad(x, y),
        ItemID::PotHeal => new_item = make_potion_heal(x, y),
        ItemID::RingStrength => new_item = make_ring_strength(x, y),
        ItemID::WandZap => new_item = make_wand_zapping(x, y),
        ItemID::AmuletYendor => new_item = make_amulet_yendor(x, y),
        ItemID::ScrollMagicMap => new_item = make_scroll_magic_map(x, y),
        ItemID::BookCantrips => new_item = make_book_cantrips(x, y),
        //ItemID::WeaponDagger => new_item = weapon_dagger(x, y),
        //_ => new_item = throw_error(-99, -99), // for testing
    }
    return new_item;
}

pub fn use_item(item_id: ItemID, obj_id: usize, objects: &mut Vec<Object>, messages: &mut Messages) {
    match item_id {
        //TODO make more generic?
        ItemID::WeaponDagger => { use_weapon_dagger(obj_id, objects);return; },
        ItemID::WeaponAxeBroad => { use_weapon_axe_broad(obj_id, objects);return; },
        ItemID::PotHeal => { use_potion_heal(obj_id, objects); return; },
        ItemID::RingStrength => { use_ring(obj_id, objects); return; },//use_ring_strength(),
        ItemID::WandZap => { use_wand_zapping(obj_id, objects, messages); return; },//use_wand_zapping(),
        ItemID::AmuletYendor => return,//use_amulet_yendor(),
        ItemID::ScrollMagicMap => { use_scroll_magic_map(obj_id, objects); return; },
        ItemID::BookCantrips => { use_book_cantrips(obj_id, objects); return; },

    }
}


// TODO streamline use funcs and make generic?

// fn use_no() {
//     //item can't be used
// }

// fn throw_error(x: u8, y: u8) -> Object {
//     let mut error = Object::new(x, y, 1, 
//         ')', "error", "fg_white", "nocolor", false, false, false, false);
//     error.item = Some(Item {
//         id: ItemID::WeaponDagger,
//         held: false,
//         slot: '0',
//         level: 1,
//         charges: 100, //use charges for weapon durability / damage - _Blessed (by your god?) dont damage? - maybe you cant bless an item until you have a god?
//         buc_status: BUC::_Normal,
//         type: None,
//     });
//     //objects.push(dagger);
//     return error;
// }

// BOOKS

fn make_book_cantrips(x: i32, y: i32) -> Object {
    let mut book = Object::new(x, y, 1, 
        '+', "Book of Cantrips", "fg_yellow", "nocolor", false, false, false, false);
    book.item = Some(Item {
        id: ItemID::BookCantrips,
        held: false,
        slot: '0',
        level: 1,
        charges: 100, //use charges for weapon durability / damage - _Blessed (by your god?) dont damage? - maybe you cant bless an item until you have a god?
        buc_status: BUC::_Normal,
        item_type: Some(Type::Book),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return book;
}

//fn read_book()


fn use_book_cantrips(_id: usize, objects: &mut Vec<Object>) {

    //read_book() - > if selected learn_spell()

    // spell level = int * 10 (or something) = each spell is worth so many spell points
    crate::rlib::spells::read_book_test(crate::rlib::spells::SpellID::_CantripBloop, objects);
}

// WEAPONS ////////////////////////////////////////////////////////////////////

fn make_weapon_dagger(x: i32, y: i32) -> Object {
    let mut dagger = Object::new(x, y, 1, 
        ')', "Dagger", "fg_white", "nocolor", false, false, false, false);
    dagger.item = Some(Item {
        id: ItemID::WeaponDagger,
        held: false,
        slot: '0',
        level: 1,
        charges: 100, //use charges for weapon durability / damage - _Blessed (by your god?) dont damage? - maybe you cant bless an item until you have a god?
        buc_status: BUC::_Normal,
        item_type: Some(Type::Weapon1h),
        body_slot: None,
        damage: 5,
        attack_speed: 10,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return dagger;
}

fn use_weapon_dagger(id: usize, objects: &mut [Object]) {
    if objects[id].item.unwrap().is_in_use() {
        unwield_weapon_dagger(id, objects);
    }
    else {
        wield_weapon_dagger(id, objects);
    }
}

fn wield_weapon_dagger(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = Some(BodySlot::RightWeapon);
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.dex += 5;
            }
        }
        BUC::_Blessed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.max_hp += 5;
                actor.hp = actor.max_hp;
            }
        }
        BUC::_Cursed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp -= 10;
            }
        }
    }
}

fn unwield_weapon_dagger(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = None;
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.dex -= 5;
            }
        }
        BUC::_Blessed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.max_hp += 5;
                actor.hp = actor.max_hp;
            }
        }
        BUC::_Cursed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp -= 10;
            }
        }
    }
}

fn make_weapon_axe_broad(x: i32, y: i32) -> Object {
    let mut axe = Object::new(x, y, 1, 
        ')', "Broad Axe", "fg_white", "nocolor", false, false, false, false);
    axe.item = Some(Item {
        id: ItemID::WeaponAxeBroad,
        held: false,
        slot: '0',
        level: 1,
        charges: 100, //use charges for weapon durability / damage - _Blessed (by your god?) dont damage? - maybe you cant bless an item until you have a god?
        buc_status: BUC::_Normal,
        item_type: Some(Type::Weapon1h),
        body_slot: None,
        damage: 5,
        attack_speed: 10,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return axe;
}

fn use_weapon_axe_broad(id: usize, objects: &mut [Object]) {
    if objects[id].item.unwrap().is_in_use() {
        unwield_weapon_axe_broad(id, objects);
    }
    else {
        wield_weapon_axe_broad(id, objects);
    }
}

fn wield_weapon_axe_broad(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = Some(BodySlot::RightWeapon);
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.dex += 5;
            }
        }
        BUC::_Blessed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.max_hp += 5;
                actor.hp = actor.max_hp;
            }
        }
        BUC::_Cursed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp -= 10;
            }
        }
    }
}

fn unwield_weapon_axe_broad(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = None;
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.dex -= 5;
            }
        }
        BUC::_Blessed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.max_hp += 5;
                actor.hp = actor.max_hp;
            }
        }
        BUC::_Cursed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp -= 10;
            }
        }
    }
}

// POTIONS ////////////////////////////////////////////////////////////////////

fn make_potion_heal(x: i32, y: i32) -> Object {
    let mut potion = Object::new(x, y, 1, 
        '!', "Potion of Healing", "fg_red", "nocolor", false, false, false, false);
    potion.item = Some(Item {
        id: ItemID::PotHeal,
        held: false,
        slot: '0',
        level: 1,
        charges: 0,
        buc_status: BUC::_Normal,
        item_type: Some(Type::Potion),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return potion;
}

fn use_potion_heal(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    objects[id].visible = false;
    objects[id].set_pos(-99, -99);
    if let Some(item) =  objects[id].item.as_mut() {
        item.held = false;
        item.charges = 0;
        item.level = 0;
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp = actor.max_hp;
            }
        }
        BUC::_Blessed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.max_hp += 5;
                //actor.hp = actor.max_hp;
            }
        }
        BUC::_Cursed => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.hp -= 10;
            }
        }
    }

}

// RINGS //////////////////////////////////////////////////////////////////////

fn which_finger(_objects: &mut [Object]) -> BodySlot{
    // TODO place holder 
    return BodySlot::LeftFinger;
}

fn use_ring(id: usize, objects: &mut [Object]) {
    if objects[id].item.unwrap().is_in_use() {
        remove_ring_strength(id, objects);
    }
    else {
        wear_ring_strength(id, objects);
    }
}

fn make_ring_strength(x: i32, y: i32) -> Object {
    let mut ring = Object::new(x, y, 1, 
        '=', "Ring of Strength", "fg_yellow", "nocolor", false, false, false, false);
    ring.item = Some(Item {
        id: ItemID::RingStrength,
        held: false,
        slot: '0',
        level: 1,
        charges: 0,
        buc_status: BUC::_Normal,
        item_type: Some(Type::Ring),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return ring;
}

fn wear_ring_strength(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    let finger = which_finger(objects);

    objects[id].visible = false;
    //objects[id].set_pos(-77, -77);
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = Some(finger);
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.strength += 10;
            }
        }
        BUC::_Blessed => {
        
        }
        BUC::_Cursed => {
        
        }
    }

}

fn remove_ring_strength(id: usize, objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    objects[id].visible = false;
    //objects[id].set_pos(-77, -77);
    if let Some(item) =  objects[id].item.as_mut() {
        item.body_slot = None;
    }
    match buc_status {
        BUC::_Normal => {
            if let Some(actor) =  objects[HERO].actor.as_mut() {
                actor.strength -= 10;
            }
        }
        BUC::_Blessed => {
        
        }
        BUC::_Cursed => {
        
        }
    }
}

// WANDS //////////////////////////////////////////////////////////////////////

fn make_wand_zapping(x: i32, y: i32) -> Object {
    let mut wand = Object::new(x, y, 1, 
        '/', "Wand of Zapping", "fg_green", "nocolor", false, false, false, false);
    wand.item = Some(Item {
        id: ItemID::WandZap,
        held: false,
        slot: '0',
        level: 1,
        charges: 5,
        buc_status: BUC::_Normal,
        item_type: Some(Type::Wand),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return wand;
}

fn use_wand_zapping(id: usize, objects: &mut [Object], messages: &mut Messages) {
    let (_level, charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    if charges < 1 {
        messages.add(format!("There are no charges left..."), "TESTCOLOR".to_string());
        return
    }
    
    if let Some(item) =  objects[id].item.as_mut() {
        item.charges -= 1;
    }
    match buc_status {
        BUC::_Normal => targeted_zap_wand(objects, messages),
        BUC::_Blessed => {},
        BUC::_Cursed => {},
    }

}

// pub fn zap_wand(mut objects: &mut [Object], mut messages: &mut Messages) {
//     messages.add(format!("Zap which direction?"), "TESTCOLOR".to_string());
//     let sx = objects[HERO].x;
//     let sy = objects[HERO].y;

//     let direction: char;
//     let direction = rlib::input::menu_keypress();
//     let zap_line: Vec<rlib::fov::Point>;

//     match direction {
//         'h' => zap_line = rlib::fov::get_line(sx, sy, 0, sy),
//         'j' => zap_line = rlib::fov::get_line(sx, sy, sx, MAP_HEIGHT),
//         'k' => zap_line = rlib::fov::get_line(sx, sy, sx, 0),
//         'l' => zap_line = rlib::fov::get_line(sx, sy, MAP_WIDTH, sy),
//         'y' => zap_line = rlib::fov::get_line(sx, sy, 0, 0),
//         'u' => zap_line = rlib::fov::get_line(sx, sy, 0, MAP_HEIGHT),
//         'b' => zap_line = rlib::fov::get_line(sx, sy, 0, MAP_HEIGHT),
//         'n' => zap_line = rlib::fov::get_line(sx, sy, MAP_WIDTH, MAP_HEIGHT),
//         _ => return,
//     }
//     //println!("dir {:?}", direction);
//     // if direction == 'h' {
//     //     let zap_line = rlib::fov::get_line(sx, sy, 0, sy);
//     //     //println!("{:?}", zap_line);
//     //     'zap: for p in zap_line {
//     //         //println!("{:?}", p);
//     //         for obj in objects.iter().position(|o| o.dlevel == objects[HERO].dlevel && o.actor.is_some() && o.x == p.x && o.y == p.y) {
//     //             println!("obj {:?}", objects[obj]);
//     //             let (hero, target) = rlib::combat::mut_two(HERO, obj, objects);
//     //             hero.zap(&mut messages, target);
//     //         }
//     //     }
//     // }

//     //  use std::{thread, time};

//     // let ten_millis = time::Duration::from_millis(10);
//     //let now = time::Instant::now();
//     use std::time::Duration;
//     use std::thread;

//     'zap: for p in zap_line {
//                 //println!("{:?}", p);
//         for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y && o.dlevel == objects[HERO].dlevel) {
//             objects[floor].bg_color = "bg_yellow".to_string();
//             let map = rlib::output::build_map(&objects);
//             rlib::output::draw_map(&map, &messages, &objects);
//             thread::sleep(Duration::from_millis(5));
//             objects[floor].bg_color = "none".to_string();
//         }
//         // //thread::sleep(ten_millis);
//         // for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y) {
//         //     //objects[floor].bg_color = "bg_yellow".to_string();
//         //     //thread::sleep(ten_millis);
//         //     // TODO make env effect function ...any color effects should be stored and reverted, altered or whatever
//         //     // like thebolt from a wand or the spice particles 
//         //     objects[floor].bg_color = "none".to_string();
//         // }
//         for obj in objects.iter().position(|o| o.dlevel == objects[HERO].dlevel && o.blocks && o.x == p.x && o.y == p.y) {
//             //println!("obj {:?}", objects[obj]);
//             if objects[obj].actor.is_some() {
//                 let (hero, target) = rlib::combat::mut_two(HERO, obj, objects);
//                 hero.zap(&mut messages, target);
//             }
//             //break 'zap;
//             break;
//         }
//         // for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y && o.dlevel == objects[HERO].dlevel) {
//         //     let map = rlib::output::build_map(&objects);
//         //     rlib::output::draw_map(&map, &messages, &objects);
//         //     thread::sleep(Duration::from_millis(4));
//         //     objects[floor].bg_color = "none".to_string();
//         // }
//             //     thread::sleep(Duration::from_millis(100));
//             // // thread::sleep(ten_millis);
//             //     for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y) {
//             //         //objects[floor].bg_color = "bg_yellow".to_string();
//             //         //thread::sleep(ten_millis);
//             //         // TODO make env effect function ...any color effects should be stored and reverted, altered or whatever
//             //         // like thebolt from a wand or the spice particles 
//             //         objects[floor].bg_color = "none".to_string();
//             //     }
//     }
//     if let Some(actor) = objects[0].actor.as_mut() {
//         actor.mana += -1;
//     }
//     //thread::sleep(Duration::from_millis(100));
//     // for id in 0..objects.len() {
//     //     if objects[id].name == "floor" && objects[id].dlevel == objects[HERO].dlevel {
//     //     objects[id].bg_color = "none".to_string();
//     //     }
//     // }
// }

pub fn targeted_zap_wand(mut objects: &mut [Object], mut messages: &mut Messages) {

    messages.add(format!("Zap which direction?"), "TESTCOLOR".to_string());
    let map = crate::rlib::output::build_map(&objects);
    crate::rlib::output::draw_map(&map, &messages, &objects);

    let x = objects[HERO].x;
    let y = objects[HERO].y;
    let mut tx = objects[HERO].x;
    let mut ty = objects[HERO].y;

    let _direction: char;
    let zap_line: Vec<crate::rlib::fov::Point>;
    objects[HERO].fg_color = "none".to_string();
    loop {
        let _direction = crate::rlib::input::menu_keypress();
        for obj in objects.iter_mut().filter(|o| o.x == tx && o.y == ty) {
            obj.bg_color = "none".to_string();
        }

        match _direction {
            'h' => tx -= 1,
            'j' => ty += 1,
            'k' => ty -= 1,
            'l' => tx += 1,
            'u' => { tx += 1; ty -= 1; },
            'y' => { tx -= 1; ty -= 1; },
            'b' => { tx -= 1; ty += 1; },
            'n' => { tx += 1; ty += 1; },
            '~' => { objects[HERO].bg_color = "bg_cyan".to_string();
                objects[HERO].fg_color = "fg_black".to_string(); return },
            'z' => break,
            'Z' => break,
            '\n' => break,
            
            _ => { objects[HERO].bg_color = "bg_cyan".to_string();
                objects[HERO].fg_color = "fg_black".to_string(); return},
        }

        for obj in objects.iter_mut().filter(|o| o.x == tx && o.y == ty) {
            obj.bg_color = "bg_cyan".to_string();
        }
        crate::rlib::cmd::list_obj_at_loc(x, y, &objects, &mut messages);
        let map = crate::rlib::output::build_map(&objects);
        crate::rlib::output::draw_map(&map, &messages, &objects);
    }
    objects[HERO].bg_color = "bg_cyan".to_string();
    objects[HERO].fg_color = "fg_black".to_string();

    zap_line = crate::rlib::fov::get_line(x, y, tx, ty);
    use std::time::Duration;
    use std::thread;

    'zap: for p in zap_line {
        for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y && o.dlevel == objects[HERO].dlevel) {
            objects[floor].bg_color = "bg_yellow".to_string();
            let map = crate::rlib::output::build_map(&objects);
            rlib::output::draw_map(&map, &messages, &objects);
            thread::sleep(Duration::from_millis(5));
            objects[floor].bg_color = "none".to_string();
        }

        for obj in objects.iter().position(|o| o.dlevel == objects[HERO].dlevel && o.blocks && o.x == p.x && o.y == p.y) {
            if objects[obj].actor.is_some() {
                let (hero, target) = rlib::combat::mut_two(HERO, obj, objects);
                hero.zap(&mut messages, target);
            }
            //break 'zap;
            break;
        }

    }


}

// AMULETS 

fn make_amulet_yendor(x: i32, y: i32) -> Object {
    let mut amulet = Object::new(x, y, 1, 
        '"', "The Amulet of Yendor", "fg_magenta", "nocolor", false, false, false, false);
    amulet.item = Some(Item {
        id: ItemID::AmuletYendor,
        held: false,
        slot: '0',
        level: 1,
        charges: 5,
        buc_status: BUC::_Normal,
        item_type: Some(Type::Amulet),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    return amulet;
}

// SCROLLS ////////////////////////////////////////////////////////////////////

fn make_scroll_magic_map(x: i32, y: i32) -> Object {
    let mut scroll = Object::new(x, y, 1, 
        '?', "Scroll of Magic Mapping", "fg_lightblue", "nocolor", false, false, false, false);
    scroll.item = Some(Item {
        id: ItemID::ScrollMagicMap,
        held: false,
        slot: '0',
        level: 1,
        charges: 1,
        buc_status: BUC::_Normal,
        item_type: Some(Type::Scroll),
        body_slot: None,
        damage: 0,
        attack_speed: 0,
        ac: 0,
        brand: 0,
        stack: 1,
    });
    scroll
}

fn use_scroll_magic_map(id: usize, mut objects: &mut [Object]) {
    let (_level, _charges, buc_status) = objects[id].item.unwrap().get_stats();
    
    objects[id].visible = false;
    objects[id].set_pos(-99, -99);
    if let Some(item) =  objects[id].item.as_mut() {
        item.held = false;
        item.charges = 0;
        item.level = 0;
    }
    match buc_status {
        BUC::_Normal => {

            for id in 0..objects.len() {
                if objects[id].item.is_none() && objects[id].actor.is_none() && objects[id].dlevel == objects[HERO].dlevel {
                    objects[id].visible = true;
                    objects[id].explored = true;
                }
            }
        }
        BUC::_Blessed => {

        }
        BUC::_Cursed => {

        }
    }

}
