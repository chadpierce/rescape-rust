use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use crate::rlib::object::Object;
use crate::hero_move_attack;
use crate::hero_run_attack;
use crate::rlib::inv;
use crate::rlib::msg::Messages;
use crate::PlayerAction::*;
use crate::rlib::dungen::make_map;
use crate::rlib::wizmode::wizmode;
//use crate::cmd::examine;
use crate::rlib::usage;
use crate::rlib::cmd;
use crate::rlib::spells;
//use crate::rlib::output::build_map;
//use crate::rlib::output::draw_map;

pub fn keypress(objects: &mut Vec<Object>, mut messages: &mut Messages) -> crate::PlayerAction {
    //working poc

        let stdin = stdin();
        let _stdout = stdout().into_raw_mode().unwrap();
    
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => return Exit,
                Key::Up | Key::Char('k') => { hero_move_attack(0, -1, objects, &mut messages); return Turn; }
                Key::Down | Key::Char('j') => { hero_move_attack(0, 1, objects, &mut messages); return Turn; }
                Key::Left | Key::Char('h') => { hero_move_attack(-1, 0, objects, &mut messages); return Turn; }
                Key::Right | Key::Char('l') => { hero_move_attack(1, 0, objects, &mut messages); return Turn; }
                Key::Char('y') => { hero_move_attack(-1, -1, objects, &mut messages); return Turn; } // up left
                Key::Char('u') => { hero_move_attack(1, -1, objects, &mut messages); return Turn; } // up right
                Key::Char('b') => { hero_move_attack(-1, 1, objects, &mut messages); return Turn; } // down left
                Key::Char('n') => { hero_move_attack(1, 1, objects, &mut messages); return Turn; } // down right
                Key::Char('K') => { hero_run_attack(0, -1, objects, &mut messages); return Turn; }
                Key::Char('J') => { hero_run_attack(0, 1, objects, &mut messages); return Turn; }
                Key::Char('H') => { hero_run_attack(-1, 0, objects, &mut messages); return Turn; }
                Key::Char('L') => { hero_run_attack(1, 0, objects, &mut messages); return Turn; }
                Key::Char('Y') => { hero_run_attack(-1, -1, objects, &mut messages); return Turn; }
                Key::Char('U') => { hero_run_attack(1, -1, objects, &mut messages); return Turn; }
                Key::Char('B') => { hero_run_attack(-1, 1, objects, &mut messages); return Turn; }
                Key::Char('N') => { hero_run_attack(1, 1, objects, &mut messages); return Turn; }
                Key::Char('.') => { hero_move_attack(0, 0, objects, &mut messages); return Turn; } // wait
                Key::Char('i') => { inv::disp_inventory(objects, &mut messages); return NoTurn; }
                Key::Char(',') => { inv::check_pick_up(objects, &mut messages); return NoTurn; }
                Key::Char('d') => { inv::drop_item(objects, &mut messages); return NoTurn; }
                Key::Char('q') => { usage::quaff_item(objects, &mut messages); return Turn; } //TODO no turn if no action!!!!
                Key::Char('r') => { usage::read_item(objects, &mut messages); return Turn; }
                Key::Char('p') => { usage::put_on_jewelry(objects, &mut messages); return Turn; }
                Key::Char('R') => { usage::remove_jewelry(objects, &mut messages); return Turn; }
                Key::Char('w') => { usage::wield_weapon(objects, &mut messages); return Turn; }
                Key::Ctrl('p') => { cmd::disp_messages(&messages); return NoTurn; }
                Key::Char('z') => { usage::zap_item(objects, &mut messages); return Turn; }
                Key::Char('Z') => { spells::zap_spell(objects, &mut messages); return Turn; }
                Key::Char('?') => { cmd::disp_help(); return NoTurn; }
                //Key::Char('Z') => { targeted_zap_wand(objects, &mut messages); return Turn; }
                Key::Char(';') => { cmd::examine(objects, &mut messages); return NoTurn; }
                Key::Char('~') => { wizmode(objects, &mut messages); return NoTurn; }
                Key::Char('>') => {  // go downstairs
                    // THIS CODE MAKES DOWNSTAIRS WORK
                    // for id in objects.iter().position(|o| o.dlevel == objects[0].dlevel && o.name == "downstair") {
                    //     if !(objects[0].x == objects[id].x && objects[0].y == objects[id].y) {
                    //         return NoTurn;
                    //     }
                    //     //obj.visible = true;
                    // }
                    objects[0].dlevel += 1;
                    let lvl_exist = objects.iter().any(|o| o.dlevel == objects[0].dlevel && o.name == "upstair");
                    if lvl_exist {
                        for id in objects.iter().position(|o| o.dlevel == objects[0].dlevel && o.name == "upstair") {
                            objects[0].x = objects[id].x;
                            objects[0].y = objects[id].y;
                        }
                    } else {
                        let new_map = make_map(objects[0].dlevel, objects);
                        objects.extend(new_map);
                    }
                    return Turn;
                }
                Key::Char('<') => {  // go upstairs
                    // THIS CODE MAKES UPSTAIRS WORK
                    // for id in objects.iter().position(|o| o.dlevel == objects[0].dlevel && o.name == "upstair") {
                    //     if !(objects[0].x == objects[id].x && objects[0].y == objects[id].y) {
                    //         return NoTurn;
                    //     }
                    //     //obj.visible = true;
                    // }
                    objects[0].dlevel -= 1;
                    for id in objects.iter().position(|o| o.dlevel == objects[0].dlevel && o.name == "downstair") {
                        objects[0].x = objects[id].x;
                        objects[0].y = objects[id].y;
                    }
                    return Turn;
                }
                _ => return NoTurn
            } 
        } 
        //FIXME this should probably not be exit
        Exit
    }

pub fn menu_keypress() -> char {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => return '~',    
            Key::Char(c) => return c,   //  println!("meh! {:?}", c),
            _ => {}
         }
        return '0';
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    return '0';
}
