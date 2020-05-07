
use crate::HERO;
use crate::rlib;
use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

pub fn examine(mut objects: &mut [Object], mut messages: &mut Messages) {
    let mut x = objects[HERO].x;
    let mut y = objects[HERO].y;

    let _direction: char;

    messages.add(format!("Look around..."), "TESTCOLOR".to_string()); //TODO why does this not appear until you move?
    let map = crate::rlib::output::build_map(&objects);
    crate::rlib::output::draw_map(&map, &messages, &objects);
    objects[HERO].fg_color = "none".to_string();

    loop {
        let _direction = rlib::input::menu_keypress();
        for obj in objects.iter_mut().filter(|o| o.x == x && o.y == y) {
            obj.bg_color = "none".to_string();
        }
        match _direction {
            'h' => x -= 1,
            'j' => y += 1,
            'k' => y -= 1,
            'l' => x += 1,
            'y' => { x -= 1; y -= 1; },
            'u' => { x += 1; y -= 1; },
            'b' => { x -= 1; y += 1; },
            'n' => { x += 1; y += 1; },
            '~' => break,
            _ => return,
        }

        for obj in objects.iter_mut().filter(|o| o.x == x && o.y == y) {
            obj.bg_color = "bg_cyan".to_string();
        }
        list_obj_at_loc(x, y, &objects, &mut messages);
        let map = rlib::output::build_map(&objects);
        rlib::output::draw_map(&map, &messages, &objects);
    }
    objects[HERO].bg_color = "bg_cyan".to_string();
    objects[HERO].fg_color = "fg_black".to_string();
}

pub fn list_obj_at_loc(x: i32, y: i32, objects: &[Object], messages: &mut Messages) {
    let names = objects
    .iter().skip(1)  //skip hero
    .filter(|obj| obj.pos() == (x, y) && obj.glyph != '.' && obj.dlevel == objects[HERO].dlevel)
    .map(|obj| obj.name.clone())
    .collect::<Vec<_>>();
    names.join(", "); // join the names, separated by commas
    if names.len() > 0 { 
        messages.add(format!("Things are here: {:?}", names), "TESTCOLOR".to_string());
    }
}

pub fn disp_messages(messages: &Messages) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 2),
        termion::cursor::Hide).unwrap();
    for (ref msg, _color) in messages.iter() {
        //carriage return needed for new line while in raw mode
        write!(stdout, "{}\n\r", msg).expect("ERROR: writing output: disp_messages");
    }
    write!(stdout, "Press any key to continue...").expect("ERROR: writing output: disp_messages");
    stdout.flush().unwrap();
    stdin().events().next();
}

pub fn disp_help() {
    let mut _c: char; // = '0';
    loop {

        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 2),
            termion::cursor::Hide).unwrap();
            //let mut selected_glyph = '-';
            write!(stdout, "{}\n\r\n\r", "Rescape Help...").expect("ERROR disp help");
            write!(stdout, "  Keymap:\n\r",).expect("ERROR disp help");
            write!(stdout, "    vi movement keys (arrow keys also work for cardinal directions)\n\r",).expect("ERROR disp help");
            write!(stdout, "      h - left      y - up + left\n\r      j - down      u - up + right\n\r      k - up        b - down + left\n\r      l - right     n - down + right \n\r",).expect("ERROR disp help");
            write!(stdout, "    commands:\n\r").expect("ERROR disp help");
            write!(stdout, "      i - display inventory\n\r").expect("ERROR disp help");
            write!(stdout, "      , - pick up item\n\r").expect("ERROR disp help");
            write!(stdout, "      d - drop item\n\r").expect("ERROR disp help");
            write!(stdout, "      p - display message history\n\r").expect("ERROR disp help");
            write!(stdout, "      ; - examine items on ground\n\r").expect("ERROR disp help");
            write!(stdout, "      w - wield weapon\n\r").expect("ERROR disp help");
            write!(stdout, "      q - quaff potion\n\r").expect("ERROR disp help");
            write!(stdout, "      z - zap wand\n\r").expect("ERROR disp help");
            write!(stdout, "      p - put on jewelry      R - remove jewelry\n\r").expect("ERROR disp help");
            write!(stdout, "      ~ - wizard mode\n\r").expect("ERROR disp help");
            write!(stdout, "      CTRL + q - QUIT GAME\n\r").expect("ERROR disp help");
            // write!(stdout, "      q - quaff potion\n\r").expect("ERROR disp help");
            // write!(stdout, "      q - quaff potion\n\r").expect("ERROR disp help");
            // write!(stdout, "      q - quaff potion\n\r").expect("ERROR disp help");
            //write!(stdout, "     y - up + left\n\r     u - up + right\n\r     b - down + left\n\r     n - down + right \n\r",).expect("ERROR disp help");
            // write!(stdout, "  d - genenerate mob (no work)\n\r",).expect("ERROR disp help");
            // write!(stdout, "  e - generate item (no work)\n\r",).expect("ERROR disp help");
            // write!(stdout, "  f - controlled blink\n\r",).expect("ERROR disp help");
    
            write!(stdout, "\n\rPress any key to continue...").expect("ERROR disp help");
            stdout.flush().unwrap();
        _c = rlib::input::menu_keypress();
        break;

    }
}