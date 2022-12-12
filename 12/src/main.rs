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

fn find_route_len(grid: &HashMap<(i64, i64), i64>, start: &[(i64, i64)], end: (i64, i64)) -> i64 {
    let mut current = VecDeque::new();
    for p in start {
        current.push_back((*p, 0));
    }
    let mut visit = HashSet::new();
    while let Some((p, steps)) = current.pop_front() {
        let h = *grid.get(&p).unwrap();
        if p == end {
            return steps;
        }
        for d in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let np = (p.0 + d.0, p.1 + d.1);
            if visit.contains(&np) {
                continue;
            }
            if *grid.get(&np).unwrap_or(&100) > (h + 1) {
                continue;
            }

            visit.insert(np);
            current.push_back((np, steps + 1));
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut s2 = Vec::new();
    for (y, l) in values.iter().enumerate() {
        let y = y as i64;
        for (x, v) in l.chars().enumerate() {
            let x = x as i64;
            let p = (x, y);
            let elev;
            if v == 'S' {
                elev = 0;
                start = p;
            } else if v == 'E' {
                elev = 25;
                end = p;
            } else {
                elev = v as i64 - 'a' as i64;
            }
            if elev == 0 {
                s2.push(p);
            }
            grid.insert(p, elev);
        }
    }
    println!("{}", find_route_len(&grid, &[start], end));
    println!("{}", find_route_len(&grid, &s2, end));
}
