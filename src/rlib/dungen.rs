use rand::Rng;
use std::cmp;
use crate::rlib::object::Object;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::HERO;
use crate::rlib::mobgen;

// pub const ROOM_MAX_SIZE: i32 = 18;
// const ROOM_MIN_SIZE: i32 = 18;
// const MAX_ROOMS: i32 = 30;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

type Floor = Vec<Vec<bool>>;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub x1: i32, pub y1: i32, pub x2: i32, pub y2: i32
}
impl Rectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rectangle {
            x1: x, y1: y, x2: x + w, y2: y + h
        }
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    
    pub fn intersects_with(&self, other: &Rectangle) -> bool {
        // returns true if rectable intersects with existing one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}
fn create_room(room: Rectangle, floor: &mut Floor) {
    // make rect tiles empty
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            floor[x as usize][y as usize] = false;
        }
    }
}

pub fn make_map(dlevel: i32, objects: &mut [Object]) ->  Vec<Object> {
    // TODO make this expand to existing object vec without returning
    let mut new_map = vec![];
    let mut floor = vec![vec![true; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    //let mut room_mobs = vec![];
    let mut rooms = vec![];
    let mut mobs = vec![];
    let mut upstair_x = -1;
    let mut upstair_y = -1;
    let mut downstair_x = -1;
    let mut downstair_y = -1;

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position - wont go beyond screen bounds
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
        let new_room = Rectangle::new(x, y, w, h);

        // make sure room doesnt intersect with others
        // no intersections
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            // center of new room
            let (new_x, new_y) = new_room.center();
            // "paint" it to the map's tiles
            create_room(new_room, &mut floor);
            if rooms.is_empty() {
                // first room. hero & upstairs goes here
                upstair_x = new_x;
                upstair_y = new_y;
                objects[HERO].set_pos(new_x, new_y);
            } 
            else {
                //downstairs in last room generated
                downstair_x = new_x;
                downstair_y = new_y;
                //generate mobs - placed here in loop so none gen in starting room
                let room_mobs = mobgen::place_mobs(dlevel, &new_room);
                mobs.extend(room_mobs);
                // all other rooms
                // connect to prev room with tunnel
                // center of previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                // random tunnel directions
                if rand::random() {
                    make_hor_tunnel(prev_x, new_x, prev_y, &mut floor);
                    make_vert_tunnel(prev_y, new_y, new_x, &mut floor);
                } else {
                    make_vert_tunnel(prev_y, new_y, prev_x, &mut floor);
                    make_hor_tunnel(prev_x, new_x, new_y, &mut floor);
                }
            }
            // append the new room to the list
            rooms.push(new_room);
        }
    }   //   ▓  ▒  ░  █  ∏  ∆  ∑   ≈  ◊  µ  π  ¿  █
    // make wall object for each 'true' tile, floor for false
    for h in 0..MAP_HEIGHT {
        for w in 0..MAP_WIDTH {
            if floor[w as usize][h as usize] {
                let new_terrain_object = Object::new(w, h, dlevel, '█', "wall", "fg_white", "nocolor", true, true, false, false);
                new_map.push(new_terrain_object);
            }
            else {
                let new_terrain_object = Object::new(w, h, dlevel, '.', "floor", "fg_white", "nocolor", false, false, false, false);
                new_map.push(new_terrain_object);

            }
        }
    }
    // create upstairs in first room
    let upstair = Object::new(upstair_x , upstair_y, dlevel, 
        '<', "upstair", "fg_black", "bg_white", false, false, true, true);
    new_map.push(upstair);
    let downstair = Object::new(downstair_x , downstair_y, dlevel, 
        '>', "downstair", "fg_black", "bg_white", false, false, false, false);
    new_map.push(downstair);
    //add mobs to object to pass back to main func, then return
    new_map.extend(mobs);
    new_map
}

fn make_hor_tunnel(x1: i32, x2: i32, y: i32, floor: &mut Floor) {
    // horizontal tunnel. min/max are in case x1 is bigger than x2
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        floor[x as usize][y as usize] = false;
    }
}

fn make_vert_tunnel(y1: i32, y2: i32, x: i32, floor: &mut Floor) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        floor[x as usize][y as usize] = false;
    }
}
