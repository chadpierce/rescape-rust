use termion::{color};
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
use crate::rlib::items;

type Map = Vec<Vec<Tile>>;

// tiles are based on objects and used to
// create the map (all objects) on screen
//#[derive(Clone, Copy, Debug)]
#[derive(Clone, Debug)]
pub struct Tile {
    glyph: char,
    fg_color: String, 
    bg_color: String,
}

impl Tile {
    pub fn new(glyph: char, fg_color: &str, bg_color: &str) -> Self {
        Tile {
            glyph: glyph,
            fg_color: fg_color.into(),
            bg_color: bg_color.into()
        }
    }
    pub fn empty() -> Self {
        Tile {
            glyph: ' ',
            //TODO fix colors
            fg_color: "Red".to_string(),
            bg_color: String::from("Black")
        }
    }
}

fn get_color(color: &str) -> String {
    
    match color{
        "fg_white" => format!("{}", color::Fg(color::White)),
        "fg_black" => format!("{}", color::Fg(color::Black)),
        "fg_red" => format!("{}", color::Fg(color::Red)),
        "fg_green" => format!("{}", color::Fg(color::Green)),
        "fg_blue"  => format!("{}", color::Fg(color::Blue)),
        "fg_magenta" => format!("{}", color::Fg(color::Magenta)),
        "fg_cyan" => format!("{}", color::Fg(color::Cyan)),
        "fg_yellow" => format!("{}", color::Fg(color::Yellow)),
        "fg_lightwhite" => format!("{}", color::Fg(color::LightWhite)),
        "fg_lightblack" => format!("{}", color::Fg(color::LightBlack)),
        "fg_lightred" => format!("{}", color::Fg(color::LightRed)),
        "fg_lightgreen" => format!("{}", color::Fg(color::LightGreen)),
        "fg_lightblue"  => format!("{}", color::Fg(color::LightBlue)),
        "fg_lightmagenta" => format!("{}", color::Fg(color::LightMagenta)),
        "fg_lightcyan" => format!("{}", color::Fg(color::LightCyan)),
        "fg_lightyellow" => format!("{}", color::Fg(color::LightYellow)),
        "bg_white" => format!("{}", color::Bg(color::White)),
        "bg_black" => format!("{}", color::Bg(color::Black)),
        "bg_red" => format!("{}", color::Bg(color::Red)),
        "bg_green" => format!("{}", color::Bg(color::Green)),
        "bg_blue"  => format!("{}", color::Bg(color::Blue)),
        "bg_magenta" => format!("{}", color::Bg(color::Magenta)),
        "bg_cyan" => format!("{}", color::Bg(color::Cyan)),
        "bg_yellow" => format!("{}", color::Bg(color::Yellow)),
        "bg_lightwhite" => format!("{}", color::Bg(color::LightWhite)),
        "bg_lightblack" => format!("{}", color::Bg(color::LightBlack)),
        "bg_lightred" => format!("{}", color::Bg(color::LightRed)),
        "bg_lightgreen" => format!("{}", color::Bg(color::LightGreen)),
        "bg_lightblue"  => format!("{}", color::Bg(color::LightBlue)),
        "bg_lightmagenta" => format!("{}", color::Bg(color::LightMagenta)),
        "bg_lightcyan" => format!("{}", color::Bg(color::LightCyan)),
        "bg_lightyellow" => format!("{}", color::Bg(color::LightYellow)),
        "clr_purple" => format!("{}", color::Bg(termion::color::Rgb(150, 50, 255))),
        _ => format!(""), // anything not an accepted color will be default terminal color
    }
}

pub fn build_map(objects: &[Object]) -> Map {
    // fill map with empty tiles, then add any objects
    let current_dlevel = objects[0].dlevel; //current floor
    let current_objects = objects.iter().filter(|obj| obj.dlevel == current_dlevel).collect::<Vec<_>>();
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    //TODO rebuild this with iter (move_by func)
    for h in 0..MAP_HEIGHT {
        for w in 0..MAP_WIDTH {
            for id in 0..current_objects.len() {
                if current_objects[id].x == w && current_objects[id].y == h {
                    if current_objects[id].visible == false && current_objects[id].explored == false {
                        // dont use actual object color here or it shows up on map!
                        map[w as usize][h as usize] = Tile::new(' ', "nocolor", "nocolor");
                    }
                    else if current_objects[id].visible == false && current_objects[id].explored == true {
                        map[w as usize][h as usize] = Tile::new(current_objects[id].glyph, "fg_magenta", "nocolor");
                    }
                    else {
                        map[w as usize][h as usize] = Tile::new(current_objects[id].glyph, &current_objects[id].fg_color, &current_objects[id].bg_color);
                        if current_objects[id].blocks { break; }
                        else if current_objects[id].item.is_some() { break; }
                        // TODO FIXME live mobs, objects, then anything else 
                        // TODO if stack invert colors
                    }
                }
            }            
        } 
    }
    map
}  

pub fn draw_map(map: &Map, messages: &Messages, objects: &[Object]) {
    use termion::raw::IntoRawMode;
    use std::io::{self, Write, stdout};

    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 2),
        termion::cursor::Hide).unwrap();

    for h in 0..MAP_HEIGHT {
        for w in 0..MAP_WIDTH {
            write!(stdout, "{fg}{bg}{glyph}{resetfg}{resetbg}", 
              //TODO should i get the colors in the build map func instead?
              fg = get_color(&map[w as usize][h as usize].fg_color),
              bg = get_color(&map[w as usize][h as usize].bg_color),
              glyph = map[w as usize][h as usize].glyph,
              resetfg = color::Fg(color::Reset),
              resetbg = color::Bg(color::Reset)).expect("ERROR: draw map");
        }
        
        //right side info panel
        //TODO make this a match statement
        //TODO handle 2h and dual wield
        let mut cur_weapon = "Bare Handed".to_string();
        for id in 0..objects.len() { //TODO FIXME this is garbage - fix it!
            if objects[id].item.is_some() && objects[id].item.unwrap().is_in_use() == true && 
            (objects[id].item.unwrap().item_type == Some(items::Type::Weapon1h) || 
            objects[id].item.unwrap().item_type == Some(items::Type::Weapon1h)){
                cur_weapon = (*objects[id].name).to_string();
            }
        }
        
        if let Some(actor) = objects[0].actor {
            if h == 1 { write!(stdout, " Hero the rogue").expect("ERROR: draw stats"); }
            if h == 2 { write!(stdout, " DLvl: {}", objects[0].dlevel).expect("ERROR: draw stats"); }
            if h == 3 { write!(stdout, " HP: {}/{}", actor.hp, actor.max_hp).expect("ERROR: draw stats"); }
            if h == 4 { write!(stdout, " Mana: {}/{}", actor.mana, actor.max_mana).expect("ERROR: draw stats"); }
            if h == 5 { write!(stdout, " Str: {}", actor.strength).expect("ERROR: draw stats"); }
            if h == 6 { write!(stdout, " Dex: {}", actor.dex).expect("ERROR: draw stats"); }
            if h == 7 { write!(stdout, " Int: {}", actor.int).expect("ERROR: draw stats"); }
            if h == 8 { write!(stdout, " Weapon: {}", cur_weapon).expect("ERROR: draw stats"); }
        write!(stdout, "\n\r").expect("ERROR: draw stats");
        }
    }
    
    let mut i = 0;
    let mut m1 = String::from("Hello!");
    let mut m2 = String::from("");
    let mut m3 = String::from("");
    let mut m4 = String::from("");
    let mut m5 = String::from("");

    for (ref msg, _color) in messages.iter().rev() {
        if i == 0 { m1 = msg.to_string(); }
        if i == 1 { m2 = msg.to_string(); }
        if i == 2 { m3 = msg.to_string(); }
        if i == 3 { m4 = msg.to_string(); }
        if i == 4 { m5 = msg.to_string(); break }
        i += 1;
    }

    write!(stdout, "{}\n\r", m5).expect("ERROR: draw messages");
    write!(stdout, "{}\n\r", m4).expect("ERROR: draw messages");
    write!(stdout, "{}\n\r", m3).expect("ERROR: draw messages");
    write!(stdout, "{}\n\r", m2).expect("ERROR: draw messages");
    write!(stdout, "{}\n\r", m1).expect("ERROR: draw messages");

    /*
    for debug
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(1000));
    //io::stdout().flush().unwrap();
    */
    io::stdout().flush().unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

/*
pub fn write_debug_log(msg: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut msg_newline: String = "\n".to_owned();
    msg_newline.push_str(msg);
    let mut file = OpenOptions::new().append(true).open("/Users/chad/git/roguelike-ideas/rdev/debug.txt").expect("cannot open file");
    file.write_all(msg_newline.as_bytes()).expect("write failed");
}
*/
