use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use crate::rlib::msg::Messages;
use crate::rlib::object::Object;
use crate::rlib::items;
use crate::rlib::input;

pub fn wield_weapon(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Weapon1h &&
        objects[id].item.unwrap().is_in_use() == false { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have no weapons...".to_string();
    let str_menu = "What would you like to wield?".to_string();
    let str_action = "Weapon at the ready: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

// pub fn unwield_weapon(mut objects: &mut [Object], mut messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
//     let mut inv_items = Vec::new();

//     for id in 0..objects.len() {
//         if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
//         objects[id].item.unwrap().item_type.is_some() &&
//         objects[id].item.unwrap().item_type.unwrap() == items::Type::Weapon1h &&
//         objects[id].item.unwrap().is_in_use() == true { 
//             inv_items.push(id);
//         }
//     }
    
//     let str_empty = "You are not holding a weaopon...".to_string();
//     let str_menu = "What would you like to put away?".to_string();
//     let str_action = "Weapon put away: ".to_string();

//     item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
// }

pub fn put_on_jewelry(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        (objects[id].item.unwrap().item_type.unwrap() == items::Type::Ring || 
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Amulet ) &&
        objects[id].item.unwrap().is_in_use() == false { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have no jewelry...".to_string();
    let str_menu = "What would you like to put on?".to_string();
    let str_action = "Put on: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

pub fn remove_jewelry(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        (objects[id].item.unwrap().item_type.unwrap() == items::Type::Ring  || 
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Amulet) &&
        objects[id].item.unwrap().is_in_use() == true { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have no jewelry...".to_string();
    let str_menu = "What would you like to put on?".to_string();
    let str_action = "Put on: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

pub fn quaff_item(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Potion { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have nothing to quaff...".to_string();
    let str_menu = "What would you like to quaff?".to_string();
    let str_action = "Quaffed: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

pub fn zap_item(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Wand { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have nothing to zap...".to_string();
    let str_menu = "What would you like to zap?".to_string();
    let str_action = "Zapped: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

pub fn read_item(objects: &mut Vec<Object>, messages: &mut Messages) { //} -> (Vec<usize>, String, String, String) {
    
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true &&
        objects[id].item.unwrap().item_type.is_some() &&
        (objects[id].item.unwrap().item_type.unwrap() == items::Type::Scroll || 
        objects[id].item.unwrap().item_type.unwrap() == items::Type::Book) { 
            inv_items.push(id);
        }
    }
    
    let str_empty = "You have nothing to read...".to_string();
    let str_menu = "What would you like to read?".to_string();
    let str_action = "Read: ".to_string();

    item_menu(objects, messages, inv_items, str_empty, str_menu, str_action);
}

fn item_menu(objects: &mut Vec<Object>, messages: &mut Messages, mut inv_items: Vec<usize>,
  str_empty: String, str_menu: String, str_action: String ) {

    if inv_items.is_empty() {
        messages.add(format!("{}", str_empty), "TESTCOLOR".to_string());

        return
    } 
    //sort by inv slot char
    inv_items.sort_by(|a, b| objects[*a as usize].item.unwrap().slot.cmp(&objects[*b as usize].item.unwrap().slot)); 

    // TODO FIXME what to do if stack exceeds 52 items?? (more than alpha limit)
    // ALSO what to do with multiple pages of items?

    let mut c: char; // = '0';
    loop {
        
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 2),
            termion::cursor::Hide).unwrap();
            write!(stdout, "{}\n\r\n\r", str_menu).expect("ERROR: item menu");
                for id in 0..inv_items.len() {
                    write!(stdout, "{} - {}\n\r", objects[inv_items[id]].item.unwrap().slot, objects[inv_items[id]].name)
                      .expect("ERROR: item menu");
                }
            write!(stdout, "\n\rWhich item?").expect("ERROR: item menu");
            stdout.flush().unwrap();
        c = input::menu_keypress();
        if c.is_ascii_alphabetic() {
            for id in 0..inv_items.len() {
                if objects[inv_items[id]].item.is_some() && objects[inv_items[id]].item.unwrap().slot == c {
                    crate::rlib::items::use_item(objects[inv_items[id]].item.unwrap().id, inv_items[id], objects, messages);
                    messages.add(format!("{} {} - {}", str_action, objects[inv_items[id]].item.unwrap().slot,
                      objects[inv_items[id]].name), "TESTCOLOR".to_string());
                }
            }
            break;
        }
        else if c == '~' {
            break;
        }
    }
}
