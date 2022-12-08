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

fn visible(
    sx: i64,
    sy: i64,
    dx: i64,
    dy: i64,
    m: &HashMap<(i64, i64), i64>,
    vis: &mut HashSet<(i64, i64)>,
) {
    let mut x = sx;
    let mut y = sy;
    let mut cur = -1;
    while let Some(&v) = m.get(&(x, y)) {
        if v > cur {
            cur = v;
            vis.insert((x, y));
        }
        x += dx;
        y += dy;
    }
}

fn visible_2(sx: i64, sy: i64, dx: i64, dy: i64, m: &HashMap<(i64, i64), i64>) -> i64 {
    let mut x = sx + dx;
    let mut y = sy + dy;
    let start = m[&(sx, sy)];

    let mut count = 0;
    while let Some(&v) = m.get(&(x, y)) {
        count += 1;
        if v >= start {
            break;
        }
        x += dx;
        y += dy;
    }
    count
}

fn scenic_score(x: i64, y: i64, m: &HashMap<(i64, i64), i64>) -> i64 {
    return visible_2(x, y, 0, 1, m)
        * visible_2(x, y, 0, -1, m)
        * visible_2(x, y, 1, 0, m)
        * visible_2(x, y, -1, 0, m);
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut grid = HashMap::new();
    let mut mx = 0;
    let mut my = 0;
    for (y, l) in values.iter().enumerate() {
        for (x, v) in l
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i64)
            .enumerate()
        {
            grid.insert((x as i64, y as i64), v);
            mx = std::cmp::max(mx, x);
        }
        my = std::cmp::max(my, y);
    }
    let mut vis = HashSet::new();
    for x in 0..=mx {
        visible(x as i64, 0, 0, 1, &grid, &mut vis);
        visible(x as i64, my as i64, 0, -1, &grid, &mut vis);
    }
    for y in 0..=my {
        visible(0, y as i64, 1, 0, &grid, &mut vis);
        visible(mx as i64, y as i64, -1, 0, &grid, &mut vis);
    }
    println!("{:?}", vis.len());
    let p2 = (0..=mx)
        .map(|x| {
            (0..=my)
                .map(|y| scenic_score(x as i64, y as i64, &grid))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("{:?}", p2);
}
