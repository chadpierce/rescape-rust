use crate::rlib;
use crate::rlib::msg::Messages;

/// objects represent everything in the game:
///   walls, floor tiles, monsters, items, the hero. everything.
//#[derive(Debug, Clone)]
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub dlevel: i32,
    pub glyph: char,
    pub name: String,
    pub fg_color: String,
    pub bg_color: String,
    pub blocks: bool,
    pub blocks_sight: bool,
    pub visible: bool,
    pub explored: bool,
    pub actor: Option<Actor>,
    pub ai: Option<Ai>,
    pub item: Option<crate::rlib::items::Item>,
    pub spell: Option<crate::rlib::spells::Spell>,
}
impl Object {
    pub fn new(x: i32, y: i32, dlevel: i32, glyph: char, name: &str, fg_color: &str, 
        bg_color: &str, blocks: bool, blocks_sight: bool, visible: bool, explored: bool) -> Self {
        Object {
            x: x,
            y: y,
            dlevel: dlevel,
            glyph: glyph,
            name: name.into(),
            fg_color: fg_color.into(),
            bg_color: bg_color.into(),
            blocks: blocks,
            blocks_sight: blocks_sight,
            visible: visible,
            explored: explored,
            actor: None,  
            ai: None,
            item: None,
            spell: None,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    // pub fn target_pos(&self) -> (i32, i32) {
    //     // TODO this is bad fix it
    //     let mut x = 0;
    //     let mut y = 0;
    //     if let Some(actor) = self.actor {
    //         if let Some(target_x) = actor.target_x { 
    //             x = target_x;
    //         }
    //         if let Some(target_y) = actor.target_y { 
    //             y = target_y;
    //         }
    //     }
    //     (x, y)
    // }
    pub fn set_target_pos(&mut self, x: i32, y: i32) {
        if let Some(actor) = self.actor.as_mut() {
            actor.target_x = Some(x);
            actor.target_y = Some(y);
        }
    }
    // pub fn clear_target_pos(&mut self) {
    //     if let Some(actor) = self.actor.as_mut() {
    //         actor.target_x = None;
    //         actor.target_y = None;
    //     }
    // }
    // distance to another object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
    // distance to actor's target
    // pub fn distance_to_target(&self) -> f32 {
    //     if let Some(actor) = self.actor {
    //         let dx: i32 = actor.target_x.unwrap();
    //         let dy: i32 = actor.target_y.unwrap();
    //         ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    //     }
    //     else { 0.0 }
    // }

    pub fn take_damage(&mut self, damage: i32, mut messages: &mut Messages) {
        // apply damage if possible
        if let Some(actor) = self.actor.as_mut() {
            
            if damage > 0 {
                actor.hp -= damage;
            }
        }
        // check for death, call the death function
        if let Some(actor) = self.actor {
            if actor.hp <= 0 {
                self.actor.unwrap().alive = false;
                actor.on_death.callback(self, &mut messages);
            }
        }
    }
    pub fn attack(&mut self, mut messages: &mut Messages, target: &mut Object) {
        // a simple formula for attack damage
        // TODO move the fomulas to combat.rs - leave the basc damage calc here?
        
        /*
        let source_rng = rand::thread_rng().gen_range(0..3);
        let target_rng = rand::thread_rng().gen_range(0..3);
        let damage = self.actor.map_or(0, |f| f.strength + source_rng) - target.actor.map_or(0, |f| f.strength + target_rng);
        */

        let damage = crate::rlib::combat::melee_attack(self, target);

        
        if damage > 0 {
            // make the target take some damage
            messages.add(format!("{} attacks {}", self.name, target.name), "TESTCOLOR".to_string());
            target.take_damage(damage, &mut messages);
        } else {
                messages.add(format!("The {} swiped at {} but missed!", self.name, target.name), "TESTCOLOR".to_string());
        }
    }

    // pub fn fire/throw

    pub fn zap(&mut self, mut messages: &mut Messages, target: &mut Object) {
        //let damage = self.actor.map_or(0, |f| f.int + source_rng) - target.actor.map_or(0, |f| f.strength + target_rng);
        //if damage > 0 {
            // make the target take some damage
        let damage = 4;
        messages.add(format!("The {} zapped {}!", self.name, target.name), "TESTCOLOR".to_string());
        target.take_damage(damage, &mut messages);
    }

    // pub fn push_object(&mut self, new_object: Object) {
    //     self.push(new_object);
    // }
}

//properties of any object that acts
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actor {
    pub max_hp: i32,
    pub hp: i32,
    pub ac: i32,
    pub strength: i32,
    pub dex: i32,
    pub int: i32,
    pub stealth: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub resist_magic: i32,
    pub resist_heat: i32,
    pub resist_cold: i32,
    pub resist_shock: i32,
    pub speed: i32,
    pub piety: i32,
    pub target_x: Option<i32>,
    pub target_y: Option<i32>,
    pub mob_inv: Option<MobInv>,
    pub alive: bool,
    pub on_death: rlib::combat::DeathCallback,
}

// a mob can hold special items
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MobInv {
    pub weapon1: Option<i32>,
    pub weapon2: Option<i32>,
    pub armor: Option<i32>,
    pub item: Option<i32>,
    }

//properties for mob ai
#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
    //Ranged,
    //Wizard,
    //Confused,
    //Scared,
    //Pacifist,
}
