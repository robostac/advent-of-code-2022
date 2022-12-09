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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Rope {
    knots: Vec<(i64, i64)>,
}

fn update(start: &mut i64, d: i64) {
    if d == 0 {
        return;
    } else if d < 0 {
        *start -= 1;
    } else {
        *start += 1;
    }
}

impl Rope {
    fn new(rope_size: usize) -> Self {
        Rope {
            knots: vec![(0, 0); rope_size],
        }
    }

    fn move_head(&mut self, dx: i64, dy: i64) {
        let mut cur = self.knots[0];
        cur.0 += dx;
        cur.1 += dy;
        self.knots[0] = cur;
        for i in 1..self.knots.len() {
            let mut next = self.knots[i];
            let tx = cur.0 - next.0;
            let ty = cur.1 - next.1;
            if tx.abs() > 1 || ty.abs() > 1 {
                update(&mut next.0, tx);
                update(&mut next.1, ty);
            }
            self.knots[i] = next;
            cur = next;
        }
    }

    fn tail(&self) -> (i64, i64) {
        return *self.knots.last().unwrap();
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    for p in [2, 10] {
        let mut rope = Rope::new(p);
        let mut visited = HashSet::new();
        visited.insert(rope.tail());
        for x in values.iter() {
            let (d, c) = x.split_once(' ').unwrap();
            let c: i64 = parse_input(c);
            let d = match d {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, -1),
                "D" => (0, 1),
                _ => panic!("UNKNOWN {}", d),
            };
            for _ in 0..c {
                rope.move_head(d.0, d.1);
                visited.insert(rope.tail());
            }
        }
        println!("{:?}", visited.len());
    }
}
