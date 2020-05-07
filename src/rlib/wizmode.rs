use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use crate::rlib::msg::Messages;
use crate::rlib::input::menu_keypress;
use crate::rlib::object::Object;
use crate::HERO;

pub fn wizmode(objects: &mut Vec<Object>, messages: &mut Messages) {

    let mut c: char; // = '0';
    loop {

        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 2),
            termion::cursor::Hide).unwrap();
            //let mut selected_glyph = '-';
            write!(stdout, "{}\n\r\n\r", "Yer a wizard:").expect("ERROR wizmode menu");
            write!(stdout, "  a - reveal map\n\r",).expect("ERROR wizmode menu");
            write!(stdout, "  b - full heal\n\r",).expect("ERROR wizmode menu");
            write!(stdout, "  c - full magic\n\r",).expect("ERROR wizmode menu");
            write!(stdout, "  d - genenerate mob (no work)\n\r",).expect("ERROR wizmode menu");
            write!(stdout, "  e - generate item (no work)\n\r",).expect("ERROR wizmode menu");
            write!(stdout, "  f - controlled blink\n\r",).expect("ERROR wizmode menu");
    
            write!(stdout, "\n\rWhat magic would you like to do??").expect("ERROR wizmode menu");
            stdout.flush().unwrap();
        c = menu_keypress();
        if c == '\n' {
            break;
        } 
        else if c.is_ascii_alphabetic() {
                    //println!("{:?}", c);
                    match c {
                        'a' => {magic_map(objects); return},
                        'b' => {full_heal(objects); return},
                        'c' => {full_mana(objects); return},
                        //'d' => magic_map(objects),
                        'e' => {generate_item(objects); return},
                        'f' => {controlled_blink(objects, messages); return},
                        //'g' => magic_map(objects),
                        //'h' => magic_map(objects),
                        _ => return,
                    }
        }
        else if c == '~' {
            break;
        }
    }
}


fn generate_item(objects: &mut Vec<Object>) {
    let new_item = crate::rlib::items::make_item(crate::rlib::items::ItemID::PotHeal, objects[HERO].x, objects[HERO].y);
    objects.push(new_item);
}

fn controlled_blink(objects: &mut Vec<Object>, messages: &mut Messages) {
    //write!(stdout, "{}", termion::cursor::Show).unwrap();
    //let mut stdout = stdout().flush();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "\n\r( this is buggy, ok! )").expect("ERROR wizmode blink");
    write!(stdout, "\n\rStart moving the cursor then hit enter...").expect("ERROR wizmode blink");
    stdout.flush().unwrap();
    //messages.add(format!("Blink where?"), "TESTCOLOR".to_string());
    //let x = objects[HERO].x;
    //let y = objects[HERO].y;
    let mut tx = objects[HERO].x;
    let mut ty = objects[HERO].y;

    let _direction: char;
    //let direction = rlib::input::menu_keypress();
    //let zap_line: Vec<rlib::fov::Point>;

    loop {
        let _direction = menu_keypress();
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
            '~' => { objects[HERO].bg_color = "bg_cyan".to_string(); return },
            'z' => break,
            'Z' => break,
            '\n' => break,
            
            _ => { objects[HERO].bg_color = "bg_cyan".to_string(); return},
        }

        for obj in objects.iter_mut().filter(|o| o.x == tx && o.y == ty) {
            obj.bg_color = "bg_cyan".to_string();
            //obj.visible = true;
        }
        //list_obj_at_loc(x, y, &objects, &mut messages);
        let map = crate::rlib::output::build_map(&objects);
        crate::rlib::output::draw_map(&map, &messages, &objects);
    }
    objects[HERO].bg_color = "bg_cyan".to_string();

    // if let Some(actor) = objects[0].actor.as_mut() {
    //     actor.po = actor.max_mana;
    // }
    objects[HERO].set_pos(tx, ty);

    //zap_line = rlib::fov::get_line(x, y, tx, ty);
    // use std::time::Duration;
    // use std::thread;

    // 'zap: for p in zap_line {
    //             //println!("{:?}", p);
    //     for floor in objects.iter().position(|o| o.name == "floor" && o.x == p.x && o.y == p.y && o.dlevel == objects[HERO].dlevel) {
    //         objects[floor].bg_color = "bg_yellow".to_string();
    //         let map = rlib::output::build_map(&objects);
    //         rlib::output::draw_map(&map, &messages, &objects);
    //         thread::sleep(Duration::from_millis(5));
    //         objects[floor].bg_color = "none".to_string();
    //     }

    //     for obj in objects.iter().position(|o| o.dlevel == objects[HERO].dlevel && o.blocks && o.x == p.x && o.y == p.y) {
    //         //println!("obj {:?}", objects[obj]);
    //         if objects[obj].actor.is_some() {
    //             let (hero, target) = rlib::combat::mut_two(HERO, obj, objects);
    //             hero.zap(&mut messages, target);
    //         }
    //         //break 'zap;
    //         break;
    //     }

    // }




}

fn full_mana(objects: &mut Vec<Object>) {
    if let Some(actor) = objects[0].actor.as_mut() {
        actor.mana = actor.max_mana;
    }
}

fn full_heal(objects: &mut Vec<Object>) {
    if let Some(actor) = objects[0].actor.as_mut() {
        actor.hp = actor.max_hp;
    }
}

fn magic_map(objects: &mut Vec<Object>) {
    for obj in objects.iter_mut() {
        obj.visible = true;
        //obj.blocks_sight = false;
        obj.explored = true;
        //obj.glyph = '%';
    }
}


/* THIS MIGHT WORK!!!
// Sample code to perform I/O:

use std::io;
use std::io::prelude::*;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();          // Reading input from STDIN
    println!("Hi, {}.", name);                          // Writing output to STDOUT
}

// Warning: Printing unwanted or ill-formatted data to output will cause the test cases to fail
*/

// Write your code here



/*
pub fn wizmode() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write_all(b"yer a wizard: ").unwrap();
    stdout.flush().unwrap();

    let pass = termion::input::TermRead::read_line(&mut stdin);

    if let Ok(Some(pass)) = pass {
        stdout.write_all(pass.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
    } else {
        stdout.write_all(b"Error\n").unwrap();
    }
}
*/

/*
pub fn wizmode2() {

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}",
    termion::cursor::Goto(1, 2),
    termion::cursor::Hide).unwrap();
    println!("{}{}", color::Fg(color::Magenta), color::Bg(color::LightWhite));
    println!("yer a wizard           \r");
    println!("what do you want to do?\n\r");
    
    //get_input();

    //let stdout = stdout();
    //let mut stdout = stdout.lock();
    //let stdin = stdin();
    
    
    //let mut stdin = stdin.lock();

    //stdout.write_all(b"=> ").unwrap();
    //stdout.flush().unwrap();

    //let cmd = termion::input::TermRead.read_line(&mut stdout);

    //if let Ok(Some(cmd)) = cmd {
        stdout.write_all(cmd.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
   // } else {
   //     stdout.write_all(b"Error\n").unwrap();
   // }
    //stdout.flush().unwrap();
    //let mut wizcmd = String::new();
    // stdin().read_line(&mut wizcmd).expect("Failed to read from stdin");

    // if let Some('\n')=wizcmd.chars().next_back() {
    //     wizcmd.pop();
    // }
    // if let Some('\r')=wizcmd.chars().next_back() {
    //     wizcmd.pop();
    // }
    //println!("You typed: {}",s);

    //write!(stdout, "Press any key to continue...");
    
    //println!("{}\n\r", wizcmd);
    println!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
    //let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    //stdin().events().next();

    //stdout.flush().unwrap();
}



    pub fn get_input2(prompt: &str) -> String{
        use std::io;
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }


fn get_input() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("=> ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("bad command!");
    // if let Some('\n')=s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r')=s.chars().next_back() {
    //     s.pop();
    // }
    println!("You typed: {}",s);
}
*/  
