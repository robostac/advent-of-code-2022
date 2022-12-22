#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::cmp::Ordering;
use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::str::FromStr;

fn parse_input<Type>(input: &str) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
{
    input.parse().unwrap()
}

// #[derive(Debug, PartialEq, Eq, Clone, Default)]

fn move_pos(s: i64, d: i64, mw: i64) -> i64 {
    let n = s + d;
    if n < 0 {
        return mw;
    }
    if n >= mw {
        return 0;
    }
    return n;
}

fn fscore(facing: &(i64, i64)) -> i64 {
    match facing {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellType {
    Normal(bool),
    TopLeft(bool),
    BottomLeft(bool),
}

impl CellType {
    fn closed(&self) -> bool {
        match self {
            CellType::BottomLeft(x) => *x,
            CellType::Normal(x) => *x,
            CellType::TopLeft(x) => *x,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Cube {
    points: HashMap<(i64, i64, i64, i64), CellType>,
    size: i64,
    face_start: [(i64, i64); 6],
    cur_pos: (i64, i64),
    facing: (i64, i64),
    face_pos: [i64; 6],
}

impl Cube {
    fn new(face_size: i64) -> Cube {
        let mut c: Cube = Default::default();
        c.size = face_size;
        c.face_pos = [0, 1, 2, 3, 4, 5];
        c
    }

    fn map_grid_front(&mut self, g: &HashMap<(i64, i64), bool>, sx: i64, sy: i64) {
        self.face_start[self.cur_face() as usize] = (sx, sy);
        let fl = self.size;
        for y in 0..fl {
            for x in 0..fl {
                let closed = g[&((sx + x), (sy + y))];
                if x == 0 && y == 0 {
                    self.points.insert((x, y, 0, 0), CellType::TopLeft(closed));
                } else if x == 0 && y == self.size - 1 {
                    self.points
                        .insert((x, y, 0, 0), CellType::BottomLeft(closed));
                } else {
                    self.points.insert((x, y, 0, 0), CellType::Normal(closed));
                }
            }
        }
    }

    fn rotate_face_vert(f: i64) -> i64 {
        match f {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            p => p,
        }
    }
    fn rotate_face_horiz(f: i64) -> i64 {
        match f {
            0 => 5,
            5 => 2,
            2 => 4,
            4 => 0,
            p => p,
        }
    }

    fn rotate_vertical(&mut self) {
        let mut new_points = HashMap::new();
        let old_face = self.face_pos;
        for i in 0..6 {
            let n = Self::rotate_face_vert(i);
            self.face_pos[n as usize] = old_face[i as usize];
        }
        for (p, v) in self.points.iter() {
            let face = Self::rotate_face_vert(p.3);
            let np = (p.0, p.2, (self.size - 1 - p.1), face);
            new_points.insert(np, *v);
        }
        self.points = new_points;
    }

    fn rotate_horiz(&mut self) {
        let mut new_points = HashMap::new();
        let old_face = self.face_pos;
        for i in 0..6 {
            let n = Self::rotate_face_horiz(i);
            self.face_pos[n as usize] = old_face[i as usize];
        }
        for (p, v) in self.points.iter() {
            let face = Self::rotate_face_horiz(p.3);
            let np = (p.2, p.1, self.size - 1 - p.0, face);
            new_points.insert(np, *v);
        }
        self.points = new_points;
    }

    fn make_move(&mut self) {
        let mut np = (
            self.cur_pos.0 + self.facing.0,
            self.cur_pos.1 + self.facing.1,
        );
        // println!("{:?} {:?} {:?}", self.cur_pos, self.facing, np);
        let mut horiz = 0;
        let mut vert = 0;
        if np.0 < 0 {
            self.rotate_horiz();
            self.rotate_horiz();
            self.rotate_horiz();
            horiz = 1;
            np.0 = self.size - 1;
        } else if np.1 < 0 {
            self.rotate_vertical();
            self.rotate_vertical();
            self.rotate_vertical();
            vert = 1;
            np.1 = self.size - 1;
        } else if np.0 == self.size {
            self.rotate_horiz();
            horiz = 3;
            np.0 = 0;
        } else if np.1 == self.size {
            self.rotate_vertical();
            vert = 3;
            np.1 = 0;
        }

        if let Some(v) = self.points.get(&(np.0, np.1, 0, 0)) {
            if v.closed() {
                self.cur_pos = np;
                // println!(
                //     "{:?} {:?} {:?} ",
                //     self.cur_pos.0 + self.face_start[self.cur_face() as usize].0,
                //     self.cur_pos.1 + self.face_start[self.cur_face() as usize].1,
                //     self.facing,
                // );
            } else {
                for _ in 0..horiz {
                    self.rotate_horiz();
                }
                for _ in 0..vert {
                    self.rotate_vertical();
                }
            }
        } else {
            // self.print_face();
            println!("BAD {:?}", self.face_pos);
            println!("BAD {:?}", np);
            panic!();
        }
    }

    fn cur_face(&self) -> i64 {
        self.face_pos[0]
    }

    fn print_face(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                if (x, y) == self.cur_pos {
                    print!("*");
                } else if let Some(v) = self.points.get(&(x, y, 0, 0)) {
                    match v {
                        CellType::BottomLeft(c) => print!("\\"),
                        CellType::Normal(c) => {
                            if *c {
                                print!(".")
                            } else {
                                print!("#")
                            }
                        }
                        CellType::TopLeft(c) => print!("/"),
                    }
                }
            }
            println!();
        }
    }

    fn build(
        &mut self,
        g: &HashMap<(i64, i64), bool>,
        sx: i64,
        sy: i64,
        side: i64,
        px: i64,
        py: i64,
    ) {
        if g.contains_key(&(sx, sy)) == false {
            return;
        }
        self.map_grid_front(g, sx, sy);
        for d in [
            (0, side, 0, 1),
            (side, 0, 1, 0),
            (-side, 0, 3, 0),
            (0, -side, 0, 3),
        ] {
            let nx = sx + d.0;
            let ny = sy + d.1;
            if nx == px && ny == py {
                continue;
            }
            for _ in 0..d.2 {
                self.rotate_horiz();
            }
            for _ in 0..d.3 {
                self.rotate_vertical();
            }
            self.build(g, nx, ny, side, sx, sy);
            for _ in d.2..4 {
                self.rotate_horiz();
            }
            for _ in d.3..4 {
                self.rotate_vertical();
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let mut grid = HashMap::new();
    let dir = values.pop().unwrap();

    for (y, l) in values.iter().enumerate() {
        for (x, v) in l.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;
            if v == '#' {
                grid.insert((x, y), false);
            } else if v == '.' {
                grid.insert((x, y), true);
            }
        }
    }
    let sx = grid
        .iter()
        .filter(|x| *x.1 == true && x.0 .1 == 1)
        .min_by_key(|y| y.0 .0)
        .unwrap()
        .0
         .0;

    let mut curpos = (sx, 0);
    let mut facing = (1, 0);
    let dir = dir.replace("R", ",R,");
    let dir = dir.replace("L", ",L,");
    let dir = dir.split(",").collect::<Vec<_>>();
    let max_width = grid.keys().max_by_key(|x| x.0).unwrap().0 + 2;
    let max_height = grid.keys().max_by_key(|x| x.1).unwrap().1 + 2;
    let side_length = (max_height - max_width).abs();
    let mut cube = Cube::new(side_length);

    let map_height = values.len() as i64 / side_length;
    let map_width = if map_height == 3 { 4 } else { 3 };
    // println!("{:?}", map_width);
    cube.build(&grid, sx, 0, side_length, -10000, -10000);

    cube.facing = facing;
    cube.cur_pos = (0, 0);
    for (i, d) in dir.iter().enumerate() {
        if *d == "L" {
            facing = (facing.1, -facing.0);
            cube.facing = (cube.facing.1, -cube.facing.0);
        } else if *d == "R" {
            facing = (-facing.1, facing.0);
            cube.facing = (-cube.facing.1, cube.facing.0);
        } else {
            let dist = parse_input::<i64>(*d);
            for _ in 0..dist {
                let mut np = curpos;
                cube.make_move();
                // cube.print_face();
                // println!();
                loop {
                    np.0 = move_pos(np.0, facing.0, max_width);
                    np.1 = move_pos(np.1, facing.1, max_height);
                    if let Some(v) = grid.get(&np) {
                        if *v {
                            curpos = np;
                        }
                        break;
                    }
                }
            }
        }
    }
    println!(
        "{}",
        1000 * (curpos.1 + 1) + 4 * (curpos.0 + 1) + fscore(&facing)
    );

    // cube.print_face();
    let top_left = cube
        .points
        .iter()
        .find(|x| x.0 .3 == 0 && *x.1 == CellType::TopLeft(x.1.closed()))
        .unwrap()
        .0;

    let tlp = (top_left.0, top_left.1);

    let rotations = if top_left.0 == 0 && top_left.1 == side_length - 1 {
        3
    } else if top_left.0 == side_length - 1 && top_left.1 == side_length - 1 {
        2
    } else if top_left.0 == side_length - 1 && top_left.1 == 0 {
        1
    } else {
        0
    };

    for i in 0..rotations {
        cube.facing = (cube.facing.1, -cube.facing.0);
        cube.cur_pos = (cube.cur_pos.1, side_length - 1 - cube.cur_pos.0);
    }
    let mut cubepos = cube.cur_pos;
    cubepos.0 += cube.face_start[cube.cur_face() as usize].0 + 1;
    cubepos.1 += cube.face_start[cube.cur_face() as usize].1 + 1;
    println!(
        "{}",
        1000 * cubepos.1 + 4 * cubepos.0 + fscore(&cube.facing)
    );
}
