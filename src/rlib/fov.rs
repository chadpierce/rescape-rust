use crate::rlib::object::Object;
use crate::HERO;
//use crate::MAP_WIDTH;
//use crate::MAP_HEIGHT;

//const FOV_DISTANCE: i32 = 8;
pub const FOV_DISTANCE: i32 = 20;

// TODO move ot struct.rs?
#[derive(Copy, Clone, Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32
} /*
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
*/

fn get_pov_perimeter_vector(hero_x: i32, hero_y: i32) -> Vec<Point> {
    let mut p = Vec::<Point>::new();
    //build square around hero - 
    let fov_x = hero_x - FOV_DISTANCE;
    let fov_y = hero_y - FOV_DISTANCE;
    let fov_w = fov_x + FOV_DISTANCE*2;
    let fov_h = fov_y + FOV_DISTANCE*2;

    for i in fov_x..fov_w {
        p.push(Point{x:i, y:fov_y});
        p.push(Point{x:i, y:fov_h});
    }
    for i in fov_y..(fov_h + 1) {
        p.push(Point{x:fov_x, y:i});
        p.push(Point{x:fov_w, y:i});
    }
    p

}
/* this is probably slower than the get_pov_perimeter_vector fn - 
    but could be useful
fn get_perimeter_vector() -> Vec<Point> {
    let mut p = Vec::<Point>::new();
    for i in (0..MAP_WIDTH).step_by(1) {
        p.push(Point{x:i as i32, y:0 as i32});
        p.push(Point{x:i as i32, y:MAP_HEIGHT as i32});
    }
    for i in (0..MAP_HEIGHT).step_by(1) {
        p.push(Point{x:0 as i32, y:i as i32});
        p.push(Point{x:MAP_WIDTH as i32, y:i as i32});
    }
    p
}
*/
pub fn calc_fov(objects: &mut [Object]) {
    let hero_x = objects[HERO].x;
    let hero_y = objects[HERO].y;
    let hero_z = objects[HERO].dlevel;
    let perimeter_vec = get_pov_perimeter_vector(hero_x, hero_y);

    for obj in objects.iter_mut().skip(1).filter(|object| object.dlevel == hero_z) {
        obj.visible = false;
    }
    //TODO filter object vec for current dlevel before passing to this function ..or filter here?
    for p in perimeter_vec {
        let points = get_line(objects[HERO].x, objects[HERO].y, p.x, p.y);
        let mut cnt = 0;        
        'line: for line in &points {
            for obj in objects.iter_mut().filter(|object| object.dlevel == hero_z && object.pos() == (line.x, line.y)) {
                obj.visible = true;
                obj.explored = true;
                if obj.actor.is_some() {
                    obj.set_target_pos(hero_x, hero_y);
                }
                if obj.blocks_sight { break 'line; }
            }
            cnt += 1;
            if cnt == FOV_DISTANCE { break 'line; }
        }
    }
}

pub fn get_line(ax: i32, ay: i32, bx: i32, by: i32) -> Vec<Point>{
    let p1: Point = Point { x: ax, y: ay };
    let p2: Point = Point { x: bx, y: by };
    let mut points = calculate_line(p1, p2);
    points.drain(0..1);  // remove hero (or line origin)
    points
}

//Bresenham's line algorithm from roguebasin
fn calculate_line(a: Point, b: Point) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    let mut x1 = a.x as i32;
    let mut y1 = a.y as i32;
    let mut x2 = b.x as i32;
    let mut y2 = b.y as i32;
    let is_steep = (y2-y1).abs() > (x2-x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;

    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2+1) {
        if is_steep {
            points.push(Point{x:y as i32, y:x as i32});
        } else {
            points.push(Point{x:x as i32, y:y as i32});
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len()/2) {
            let end = points.len()-1;
            points.swap(i, end-i);
        }
    }
    points
}

// fn pause() {
//     use std::io::{stdin, stdout, Read, Write};
// let mut stdout = stdout();
// stdout.write(b"Press Enter to continue...").unwrap();
// stdout.flush().unwrap();
// stdin().read(&mut [0]).unwrap();
// }
