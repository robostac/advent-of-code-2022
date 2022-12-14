#![allow(dead_code, unused_macros, unused_imports)]

use std::cell::RefCell;
use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

fn parse_input<Type>(input: &str) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
{
    input.trim().parse().unwrap()
}

fn parse(s: &str) -> (i64, i64) {
    let (a, b) = s.split_once(',').unwrap();
    (parse_input(a), parse_input(b))
}

fn direction(x: i64) -> i64 {
    match 0.cmp(&x) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
    }
}

fn add_sand(grid: &HashSet<(i64, i64)>, my: i64, p2: bool) -> Option<(i64, i64)> {
    let mut sx = 500;
    let mut sy = 0;
    if grid.contains(&(sx, sy)) {
        return None;
    }
    loop {
        if p2 == false && sy > my {
            break;
        }

        let mut found = false;
        for dx in [0, -1, 1] {
            if grid.contains(&(sx + dx, sy + 1)) == false {
                found = true;
                sx += dx;
                sy += 1;
                break;
            }
        }

        if found == false || (p2 && sy == my + 1) {
            return Some((sx, sy));
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let mut grid = HashSet::new();
    let mut ly = 0;
    for p in values {
        let c = p.split(" -> ").collect::<Vec<_>>();
        let mut last: Option<(i64, i64)> = None;
        for x in c {
            let pp = parse(x);
            ly = std::cmp::max(pp.1, ly);
            if let Some(prev) = last {
                let dx = direction(prev.0 - pp.0);
                let dy = direction(prev.1 - pp.1);

                let mut xx = prev.0;
                let mut yy = prev.1;
                loop {
                    grid.insert((xx, yy));
                    if xx == pp.0 && yy == pp.1 {
                        break;
                    }
                    xx += dx;
                    yy += dy;
                }
            }
            last = Some(pp);
        }
    }
    let mut c = 0;
    for p2 in [false, true] {
        while let Some(p) = add_sand(&grid, ly, p2) {
            c += 1;
            grid.insert(p);
        }
        println!("{}", c);
    }
}
