use crate::rlib::object::Object;
use crate::rlib::msg::Messages;
//use crate::HERO;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
//use std::collections::VecDeque; // unused 
use crate::rlib::fov::Point;
use rand::seq::SliceRandom;


// NOTES
// paths are a little crazy because they are completely random
// mobs should probably beeline when they can
// there are other quirks as well

// the following mit course was used to develop this:
// https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-006-introduction-to-algorithms-fall-2011/lecture-videos/lecture-13-breadth-first-search-bfs/

#[derive(Copy, Clone, Debug)]
pub struct PathPoint{
    pub i: i32,
    pub x: i32,
    pub y: i32,
}
// impl PathPoint {
//     pub fn new(i: i32, x: i32, y: i32) -> Self {
//         PathPoint {
//             i: i,
//             x: x,
//             y: y,
//         }
//     }
//     pub fn pos(&self) -> (i32, i32, i32) {
//         (self.x, self.y, self.i)
//     }
//     pub fn set_pos(&mut self, x: i32, y: i32, i: i32) {
//         self.i = i;
//         self.x = x;
//         self.y = y;
//     }
// }

fn is_adjacent_to(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {

    if tx == sx - 1 && ty == sy - 1 { true }
    else if tx == sx && ty == sy - 1 { true }
    else if tx == sx + 1 && ty == sy - 1 { true }
    else if tx == sx + 1 && ty == sy { true }
    else if tx == sx + 1 && ty ==  sy + 1 { true }
    else if tx == sx && ty == sy + 1 { true }
    else if tx == sx - 1 && ty == sy + 1 { true }
    else if tx == sx - 1 && ty == sy { true }
    else { false }
}

fn get_neighbors(source_x: i32, source_y: i32, grid: &Vec<Vec<bool>>, level: &mut Vec<Vec<i32>>) -> Vec<Point> {
    let mut moves = Vec::<Point>::new();
    // get surrounding cells, clockwise
    moves.push(Point{x: source_x - 1, y: source_y - 1});
    moves.push(Point{x: source_x, y: source_y - 1});
    moves.push(Point{x: source_x + 1, y: source_y - 1});
    moves.push(Point{x: source_x + 1, y: source_y});
    moves.push(Point{x: source_x + 1, y: source_y + 1});
    moves.push(Point{x: source_x, y: source_y + 1});
    moves.push(Point{x: source_x - 1, y: source_y + 1});
    moves.push(Point{x: source_x - 1, y: source_y});    
    // remove blocked cells
    moves.retain(|m| grid[m.x as usize][m.y as usize] == false); //TODO combine these
    moves.retain(|m| level[m.x as usize][m.y as usize] == -1);
    moves
}

fn get_bfs_grid(objects: &mut [Object]) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let current_objects = objects.iter()
      .filter(|obj| obj.dlevel == objects[0].dlevel).collect::<Vec<_>>();
    for h in 0..MAP_HEIGHT {
        for w in 0..MAP_WIDTH {
            for id in 0..current_objects.len() { 
                if current_objects[id].blocks && current_objects[id].blocks_sight && current_objects[id].x == w 
                && current_objects[id].y == h {
                    grid[w as usize][h as usize] = true;
                } 
            }
        }
    } 
    grid
}

pub fn bfs_test(id: usize, sx: usize, sy: usize, tx: usize, ty: usize, objects: &mut [Object], mut messages: &mut Messages) {
    let grid = get_bfs_grid(objects);
    let mut level = vec![vec![-1; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut parent = vec![vec![(-1, -1); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut i: i32 = 2; // level number

    level[sx][sy] = 0;
    let mut frontier = get_neighbors(sx as i32, sy as i32, &grid, &mut level);
    for f in &frontier {
        level[f.x as usize][f.y as usize] = 1;
        parent[f.x as usize][f.y as usize] = (sx as i32, sy as i32);
    }
    while frontier.len() > 0 {  
        let mut next = Vec::<Point>::new();
        for f in &frontier {
            let neighbors = get_neighbors( f.x as i32, f.y as i32, &grid, &mut level);
            for n in &neighbors {
                if is_adjacent_to(n.x as i32, n.y as i32, tx as i32, ty as i32) {
                    level[tx][ty] = i + 1;
                    parent[tx][ty] = (n.x as i32, n.y as i32);
                }
                level[n.x as usize][n.y as usize] = i;
                parent[n.x as usize][n.y as usize] = (f.x, f.y);
                next.push(Point{x: n.x, y: n.y});
                //for testing 
                // for id in 0..objects.len() { 
                //     if !objects[id].ai.is_some() && objects[id].dlevel == objects[0].dlevel 
                //     && objects[id].x == n.x && objects[id].y == n.y {
                //         //objects[id].bg_color = "bg_yellow".to_string();
                //         match level[n.x as usize][n.y as usize] {
                //             0 => objects[id].glyph = '0',
                //             1 => objects[id].glyph = '1',
                //             2 => objects[id].glyph = '2',
                //             3 => objects[id].glyph = '3',
                //             4 => objects[id].glyph = '4',
                //             5 => objects[id].glyph = '5',
                //             6 => objects[id].glyph = '6',
                //             7 => objects[id].glyph = '7',
                //             8 => objects[id].glyph = '8',
                //             9 => objects[id].glyph = '9',
                //             10 => objects[id].glyph = 'A',
                //             11 => objects[id].glyph = 'B',
                //             12 => objects[id].glyph = 'C',
                //             13 => objects[id].glyph = 'D',
                //             14 => objects[id].glyph = 'E',
                //             15 => objects[id].glyph = 'F',
                //             _ => objects[id].glyph = 'x',
                //         }
                //     }
                // }
            }
        }
        frontier = next;
        i += 1;
    }

//     for id in 0..objects.len() { 
//         if !objects[id].ai.is_some() && objects[id].dlevel == objects[HERO].dlevel {
//             objects[id].bg_color = "bg_none".to_string();
//         }
//    }

    let mut path_tx = tx;
    let mut path_ty = ty;

    while !is_adjacent_to(path_tx as i32, path_ty as i32, sx as i32, sy as i32) {

        let mut moves = Vec::<Point>::new();
        // get surrounding cells, clockwise
        moves.push(Point{x: path_tx as i32 - 1, y: path_ty as i32 - 1});
        moves.push(Point{x: path_tx as i32, y: path_ty as i32 - 1});
        moves.push(Point{x: path_tx as i32 + 1, y: path_ty as i32 - 1});
        moves.push(Point{x: path_tx as i32 + 1, y: path_ty as i32});
        moves.push(Point{x: path_tx as i32 + 1, y: path_ty as i32 + 1});
        moves.push(Point{x: path_tx as i32, y: path_ty as i32 + 1});
        moves.push(Point{x: path_tx as i32 - 1, y: path_ty as i32 + 1});
        moves.push(Point{x: path_tx as i32 - 1, y: path_ty as i32});    
        // remove blocked cells
        moves.retain(|m| grid[m.x as usize][m.y as usize] == false);

        let mut levels = vec![];
        let mut _shortest = 0;
        for id in 00..moves.len() {
            levels.push(level[moves[id].x as usize][moves[id].y as usize]);
            if levels[id] == -1 {
                levels[id] = MAP_HEIGHT * MAP_WIDTH; //unprocessed tile is never _shortest
            }
        }
        let min_level = levels.iter().min();
        match min_level {
            Some(min) => _shortest = *min,
            None      => _shortest = MAP_HEIGHT * MAP_WIDTH, //never _shortest
        }
        moves.retain(|m| level[m.x as usize][m.y as usize] == _shortest);
        if moves.len() != 0 {
            let the_move = moves.choose(&mut rand::thread_rng()).unwrap();
            path_tx = the_move.x as usize;
            path_ty = the_move.y as usize;
        }
        else {
            path_tx = tx; // TODO this is kind of a hack maybe fix?
            path_ty = ty;
            break;
        }
    }
    let dx = (path_tx - sx) as i32;
    let dy = (path_ty - sy) as i32;
    // DEBUG messages.add(format!("{:?} (m:{:?},{:?}) -> {:?},{:?} d:{:?},{:?}, last:{:?},{:?}",
    //id, objects[id].x, objects[id].y, objects[0].x, objects[0].y, dx, dy, path_tx, path_ty), "RED".to_string());
    crate::move_by(id, dx, dy, objects, &mut messages);
}
