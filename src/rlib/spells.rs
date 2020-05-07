use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
//use crate::HERO;
//use crate::rlib;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spell {
    pub id: SpellID,
    pub known: bool,
    //pub tick_learned: i32,
        
}

// impl Spell {
//     fn learn(&self) {
//         //check if spell object exists - add if not

//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpellID {
    _CantripBloop,
}

// pub fn read_book(id: SpellID, objects: &mut Vec<Object>) {


// }


pub fn read_book_test(_id: SpellID, objects: &mut Vec<Object>) {
    let spell = make_spell_object(SpellID::_CantripBloop);
    objects.push(spell);

    learn_spell(SpellID::_CantripBloop, objects);
}


pub fn make_spell_object(_id: SpellID) -> Object {
    let mut spell = Object::new(-1, -1, -1, 
        '+', "Bloop", "", "", false, false, false, false);
    spell.spell = Some(Spell {
        id: SpellID::_CantripBloop,
        known: false,    
     });
    return spell;
}

pub fn learn_spell(spell_id: SpellID, objects: &mut [Object]) {
    for id in 0..objects.len() {
        if objects[id].spell.is_some() && objects[id].spell.unwrap().id == spell_id { 
            objects[id].spell.unwrap().known = true;
        }
    }

    // let obj_id = objects.iter().position(|obj| obj.spell.unwrap().id == spell_id);//.collect::<Vec<_>>();

    // objects[obj_id as usize].spell.unwrap().known = true;
    // //.spell.unwrap().known.unwrap() = true;

}

pub fn zap_spell(objects: &mut [Object], messages: &mut Messages) {

    let is_known = objects.iter().any(|obj| obj.spell.is_some() && obj.spell.unwrap().id == SpellID::_CantripBloop);
    if is_known {
        messages.add(format!("BLOOOOOOOOOOOOP"), "RED".to_string());
    }
    else {
        messages.add(format!("NO BLOOP"), "RED".to_string());
    }
    
}
