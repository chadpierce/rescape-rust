use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use crate::HERO;
use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
use crate::rlib::input::menu_keypress;

// this is set for a max of 26 items, add UPPERs for 52
static INV_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub fn disp_inventory(objects: &mut [Object], messages: &mut Messages) {
    let mut items = vec![];

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true { 
            items.push(id);
        }
        else {
            if id == objects.len() && items.is_empty() {
                messages.add(format!("you are not holding any items..."), "TESTCOLOR".to_string());
                return
            } 
        }
    }
    if items.is_empty() {
        messages.add(format!("you are not holding any items..."), "TESTCOLOR".to_string());
        return
    }   
    //sort by inv slot char
    items.sort_by(|a, b| objects[*a as usize].item.unwrap().slot.cmp(&objects[*b as usize].item.unwrap().slot));
    let mut _in_use = "".to_string();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 2),
        termion::cursor::Hide).unwrap();
        write!(stdout, "{}\n\r\n\r", "Items you are holding:").expect("ERROR: inventory menu generation");
        for id in 0..items.len() {

            if objects[items[id]].item.unwrap().is_in_use() {
                _in_use = "(in use)".to_string();
            }
            else if !objects[items[id]].item.unwrap().is_in_use(){
                _in_use = "".to_string();
            }
            // TODO fix this - either rename in_use or make separate var
            else if objects[items[id]].item.unwrap().item_type == Some(crate::rlib::items::Type::Wand) {
                _in_use = format!("({} charges)" ,objects[items[id]].item.unwrap().charges.to_string());
            }
            else {
                _in_use = "".to_string();
            }

            write!(stdout, "{} - {} {}\n\r", objects[items[id]].item.unwrap().slot, objects[items[id]].name, _in_use)
              .expect("ERROR: inventory menu generation");
        }
        write!(stdout, "\n\rWhich item?").expect("ERROR: inventory menu generation");
        stdout.flush().unwrap();
        stdin().events().next();
}

pub fn check_pick_up(objects: &mut [Object], messages: &mut Messages) {
    let x = objects[HERO].x;
    let y = objects[HERO].y;
    let mut floor_items = Vec::new();       //items on floor
    let mut inv_items = Vec::new();         //items currently in inv
    let mut selected_items = Vec::new();    //is menu item selected
    let mut alpha_items = Vec::new();       //is inv slot taken
    let menu_little_a: u8 = 97;
    let menu_big_a: u8 = 65;  //for 52 inv chars...but this is not currently in use
    
    //get current inv items (for length)
    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true { 
            inv_items.push(id);
        }
    }
    if inv_items.len() >= INV_CHARS.len() {
        messages.add(format!("inventory full"), "TESTCOLOR".to_string());
        return
    }
    //get items on floor
    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].pos() == (x, y) && objects[id].dlevel == objects[HERO].dlevel { 
            floor_items.push(id);
            selected_items.push(false);  //populating vec with false
        }
    }
    if floor_items.is_empty() {
        messages.add(format!("nothing here..."), "TESTCOLOR".to_string());
        return
    }
    else if floor_items.len() == 1 {
        do_add_item(floor_items[0], objects);
        messages.add(format!("picked up: {} - {}", objects[floor_items[0]].item.unwrap().slot, 
          objects[floor_items[0]].name), "TESTCOLOR".to_string());
        return
    }  

    for id in 0..floor_items.len() {
        if id <= 26 {
            alpha_items.push((id as u8 + menu_little_a) as char);
        }
        else if id > 26 {
            alpha_items.push((id as u8 + menu_big_a) as char);
        }
    }
    let mut c: char; // = '0';
    loop {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 2),
            termion::cursor::Hide).unwrap();
            let mut _selected_glyph = '-';
            write!(stdout, "{}\n\r\n\r", "Items at your feet:").expect("ERROR: item pick up");
                for id in 0..floor_items.len() {
                    if selected_items[id] == true {
                        _selected_glyph = '+';
                    }
                    else {
                        _selected_glyph = '-';
                    }
                if id <= 26 {
                    write!(stdout, "{} {} {}\n\r", (menu_little_a + id as u8) as char, _selected_glyph,
                      objects[floor_items[id]].name).expect("ERROR: item pick up");
                }
                else if id > 26 {
                    write!(stdout, "{} {} {}\n\r", (menu_big_a + id as u8) as char, _selected_glyph,
                      objects[floor_items[id]].name).expect("ERROR: item pick up");
                }
            }
            write!(stdout, "\n\rWhich item?").expect("ERROR: item pick up");
            stdout.flush().unwrap();
        c = menu_keypress();
        if c == ',' || c == '\n' {
            //do final stuff
            for id in 0..floor_items.len() {
                if selected_items[id] == true {
                    do_add_item(floor_items[id], objects);
                    messages.add(format!("picked up: {} - {}", objects[floor_items[id]].item.unwrap().slot,
                      objects[floor_items[id]].name), "TESTCOLOR".to_string());
                }
            }
            break;
        } 
        else if c.is_ascii_alphabetic() {
            for id in 0..floor_items.len() {
                if alpha_items[id] == c {
                    if selected_items[id] == true {
                        selected_items[id] = false;
                    } else {
                        selected_items[id] = true;
                    }
                }
            }
        }
        else if c == '~' {
            break;
        }
        else {
        }

    }
}

pub fn drop_item(objects: &mut [Object], messages: &mut Messages) {
    let mut selected_items = Vec::new();
    let mut inv_items = Vec::new();

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true { 
            inv_items.push(id);
            selected_items.push(false);  //populating vec with false
        }
    }
    if inv_items.is_empty() {
        messages.add(format!("you are not holding any items..."), "TESTCOLOR".to_string());
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
            let mut _selected_glyph = '-';
            write!(stdout, "{}\n\r\n\r", "What would you like to drop?:").expect("ERROR: item drop");
                for id in 0..inv_items.len() {
                    if selected_items[id] == true {
                        _selected_glyph = '+';
                    }
                    else {
                        _selected_glyph = '-';
                    }
                write!(stdout, "{} {} {}\n\r", objects[inv_items[id]].item.unwrap().slot, _selected_glyph,
                  objects[inv_items[id]].name).expect("ERROR: item drop");
            }
            write!(stdout, "\n\rWhich item?").expect("ERROR: item drop");
            stdout.flush().unwrap();
        c = menu_keypress();
        if c == 'd' || c == '\n' {
            //do final stuff
            for id in 0..inv_items.len() {
                if selected_items[id] == true {
                    do_drop_item(inv_items[id], objects);
                    messages.add(format!("dropped: {} - {}", objects[inv_items[id]].item.unwrap().slot,
                      objects[inv_items[id]].name), "TESTCOLOR".to_string());
                }
            }
            break;
        } 
        else if c.is_ascii_alphabetic() {
            for id in 0..inv_items.len() {
                if objects[inv_items[id]].item.is_some() && objects[inv_items[id]].item.unwrap().slot == c {
                    if selected_items[id] == true {
                        selected_items[id] = false;
                    } else {
                        selected_items[id] = true;
                    }
                }
            }
        }
        else if c == '~' {
            break;
        }
    }
}

fn is_cur_slot_open(id: usize, objects: &[Object]) -> bool {
    return !objects.iter().any(|obj| obj.item.is_some() && obj.item.unwrap().held == true && obj.item.unwrap().slot == objects[id].item.unwrap().slot);
}

fn get_next_avail_slot(objects: &[Object]) -> char {
    let mut slots_in_use = Vec::new();
    let mut is_char_used = vec![false; INV_CHARS.len()];

    for id in 0..objects.len() {
        if objects[id].item.is_some() && objects[id].item.unwrap().held == true { 

            slots_in_use.push(objects[id].item.unwrap().slot);
        }
    }
    if slots_in_use.len() == 0 {
        return 'a'
    }
    slots_in_use.sort();
    for i in 0..INV_CHARS.len() {
        for s in 0..slots_in_use.len() {
            if INV_CHARS[i] == slots_in_use[s] {
                is_char_used[i] = true;
            }
        }
    }
    for id in 0..is_char_used.len() {
        if is_char_used[id] == false { 
            return INV_CHARS[id]
        }
    }
    return '%' // inv full 
}

fn do_add_item(id: usize, mut objects: &mut [Object]) {
    let new_slot: char;
    if objects[id].item.is_some() && objects[id].item.unwrap().slot != '0' && is_cur_slot_open(id, objects) == true {
        new_slot = objects[id].item.unwrap().slot;
    }
    else {
        new_slot = get_next_avail_slot(objects);
        if new_slot == '%' {
            println!("inventory full!");
            return
        }
    }
    if let Some(item) = objects[id].item.as_mut() {
        item.held = true;
        item.slot = new_slot;  //'a'; // TODO get_next_avail_slot()
    }
    objects[id].set_pos(-1, -1);
    objects[id].visible = false;
    objects[id].explored = false;
}

fn do_drop_item(id: usize, mut objects: &mut [Object]) {
    if let Some(item) =  objects[id].item.as_mut() {
        item.held = false;
    }
    objects[id].set_pos(objects[HERO].x, objects[HERO].y);
    objects[id].dlevel = objects[HERO].dlevel;
    objects[id].visible = true;
    objects[id].explored = true;
}
