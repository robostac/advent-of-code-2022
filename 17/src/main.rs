#![allow(dead_code, unused_macros, unused_imports)]

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
    input.trim().parse().unwrap()
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn dx(&self) -> i64 {
        match self {
            Jet::Left => -1,
            Jet::Right => 1,
        }
    }
}

fn try_move(
    r: &[(i64, i64)],
    dx: i64,
    dy: i64,
    grid: &HashSet<(i64, i64)>,
) -> Option<Vec<(i64, i64)>> {
    let rr = r
        .iter()
        .map(|(xx, yy)| (*xx + dx, *yy + dy))
        .collect::<Vec<_>>();
    for p in rr.iter() {
        if grid.contains(p) {
            return None;
        }
        if p.0 < 0 || p.0 >= 7 {
            return None;
        }
        if p.1 > 0 {
            return None;
        }
    }
    Some(rr)
}

fn print(grid: &HashSet<(i64, i64)>, top: i64) {
    for y in top..=0 {
        for x in 0..7 {
            if grid.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let rocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, -1), (1, -2), (1, 0), (1, -1), (2, -1)],
        vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        vec![(0, 0), (1, 0), (0, -1), (1, -1)],
    ];
    let mut jets = values[0]
        .chars()
        .map(|x| if x == '<' { Jet::Left } else { Jet::Right })
        .collect::<VecDeque<_>>();

    let mut top = 0;
    let mut grid = HashSet::new();
    let p1limit = 2022;
    let p2limit = 500000;

    let mut last = HashMap::new();
    let mut order = Vec::new();
    let mut lookup = HashMap::new();
    let mut height = Vec::new();
    for i in 0..p2limit {
        if i == p1limit {
            println!("{:}", top);
        }
        let mut r = rocks[i % rocks.len()].clone();
        let sx = 2;
        let sy = top - 3;
        for x in r.iter_mut() {
            x.0 += sx;
            x.1 += sy;
        }
        loop {
            let p = jets.pop_front().unwrap();
            jets.push_back(p.clone());
            if let Some(rr) = try_move(&r, p.dx(), 0, &grid) {
                r = rr;
            }
            if let Some(rr) = try_move(&r, 0, 1, &grid) {
                r = rr;
            } else {
                for x in r {
                    grid.insert(x);
                    top = std::cmp::min(top, x.1 - 1);
                    last.insert(x.1, i);
                }

                let mut v = 0;
                for y in top..(top + 5) {
                    let yy = (y - top).abs();
                    for x in 0..7 {
                        if grid.contains(&(x, y)) {
                            v |= 1i64 << (x + 7 * yy);
                        }
                    }
                }
                let l = lookup.len();
                let key = *lookup.entry(v).or_insert(l);
                order.push(key);
                break;
            }
        }
        height.push(top);
    }

    let mut found = false;
    for i in 0..order.len() {
        for j in (i + 5)..order.len() {
            let l = j - i;
            let k = j + l;
            if k + l > order.len() {
                break;
            }
            if order[i..j] == order[j..k] && order[k..(k + l)] == order[j..k] {
                found = true;
                let tgt = 1000000000000 - 1; //rocks are 0 indexed
                let extra = (tgt - i) % l;
                let i = i + extra;
                let j = j + extra;
                let ncycles = (tgt - i) / l;
                println!("{}", height[i] + ncycles as i64 * (height[j] - height[i]));
                break;
            }
        }
        if found {
            break;
        }
    }
}
