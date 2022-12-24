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

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn find_route(
    blizzards: &HashSet<(i64, i64, i64)>,
    start: (i64, i64),
    st: i64,
    end: (i64, i64),
    cycle_length: i64,
) -> i64 {
    let mut cur = VecDeque::new();
    cur.push_front((start.0, start.1, st));
    // let mut visit = HashSet::new();
    let mut bad = blizzards.clone();
    while let Some((px, py, t)) = cur.pop_front() {
        let nt = t + 1;
        // println!("{} {} {}", px, py, t);
        for d in [(0, 1), (-1, 0), (0, -1), (1, 0), (0, 0)] {
            let np = (px + d.0, py + d.1, nt % cycle_length);
            if np.1 < 0 {
                continue;
            }
            if bad.insert(np) == false {
                continue;
            }
            if np.0 == end.0 && np.1 == end.1 {
                return nt;
            }
            cur.push_back((np.0, np.1, nt));
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut blizzards = Vec::new();
    for (y, l) in values.iter().enumerate() {
        for (x, v) in l.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;
            if v == '>' {
                blizzards.push(((x, y), (1, 0)));
            } else if v == '<' {
                blizzards.push(((x, y), (-1, 0)));
            } else if v == 'v' {
                blizzards.push(((x, y), (0, 1)));
            } else if v == '^' {
                blizzards.push(((x, y), (0, -1)));
            } else if v == '#' {
                blizzards.push(((x, y), (0, 0)));
            }
        }
    }
    let width = values[0].len() as i64;
    let height = values.len() as i64;
    let start = (1i64, 0i64);
    let end = (width - 2, height - 1);
    let cycle_length = lcm(width - 2, height - 2);
    let mut bad = HashSet::new();
    for (p, _) in blizzards.iter() {
        bad.insert((p.0, p.1, 0));
    }

    for i in 0..cycle_length {
        let mut new_blizzards = Vec::new();
        for (p, d) in blizzards.iter() {
            let mut np = (p.0 + d.0, p.1 + d.1);
            if d.0 == -1 && np.0 == 0 {
                np.0 = width - 2;
            }
            if d.1 == -1 && np.1 == 0 {
                np.1 = height - 2;
            }
            if d.0 == 1 && np.0 == width - 1 {
                np.0 = 1;
            }
            if d.1 == 1 && np.1 == height - 1 {
                np.1 = 1;
            }
            new_blizzards.push((np, *d));
            bad.insert((np.0, np.1, i + 1));
        }
        blizzards = new_blizzards;
    }

    let p1 = find_route(&bad, start, 0, end, cycle_length);
    println!("{:?}", p1);

    let p2a = find_route(&bad, end, p1, start, cycle_length);
    // println!("{:?}", p2a);
    let p2b = find_route(&bad, start, p2a, end, cycle_length);
    println!("{:?}", p2b);
}
