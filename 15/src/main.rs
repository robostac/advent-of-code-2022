#![allow(dead_code, unused_macros, unused_imports)]

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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Self) -> i64 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut pairs = Vec::new();
    let mut beacons = Vec::new();
    for x in values {
        let x = x.replace(":", " ");
        let x = x.replace(",", " ");
        let c = x
            .split_ascii_whitespace()
            .filter_map(|y| {
                if let Some((_, n)) = y.split_once("=") {
                    Some(parse_input::<i64>(n))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let pa = Point::new(c[0], c[1]);
        let pb = Point::new(c[2], c[3]);
        let d = pb.distance(&pa);
        pairs.push((pa, d));
        beacons.push(pb);
    }
    let row = 2000000;
    let mut hm = HashSet::new();
    for (pb, od) in pairs.iter() {
        let mut d = *od;
        let dy = (row - pb.y);
        d -= dy.abs();
        if d < 0 {
            continue;
        }

        for x in (pb.x - d)..=(pb.x + d) {
            hm.insert(x);
        }
    }
    for p in beacons.iter() {
        if p.y == row {
            hm.remove(&p.x);
        }
    }
    println!("{:?}", hm.len());

    let search_max = 4000000;
    let mut ans = None;
    for (pb, od) in pairs.iter() {
        let BD = od + 1;
        for y in 0..=(BD) {
            let x = BD - y;
            for d in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let xx = d.0 * (pb.x + x);
                let yy = d.1 * (pb.y + y);
                if xx < 0 || xx > search_max || yy > search_max || yy < 0 {
                    continue;
                }
                let np = Point::new(xx, yy);
                if pairs.iter().all(|(p, y)| p.distance(&np) > *y) {
                    ans = Some(np);
                    break;
                }
            }
        }
        if ans.is_some() {
            break;
        }
    }
    if let Some(p) = ans {
        println!("{:?}", p.x * search_max + p.y);
    }
}
