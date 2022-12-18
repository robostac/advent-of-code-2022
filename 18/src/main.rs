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
// #[derive(Debug, PartialEq, Eq, Clone)]

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<i64>> = stdin
        .lock()
        .lines()
        .map(|input| {
            input
                .unwrap()
                .split(",")
                .map(|x| parse_input::<i64>(x))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut surface = values.len() * 6;
    let mut position = HashSet::new();
    let mut mp = 0;
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            let mut md = 0;
            for k in 0..3 {
                mp = std::cmp::max(mp, values[i][k]);
                md += (values[i][k] - values[j][k]).abs();
            }
            if md == 1 {
                surface -= 2;
            }
        }
        position.insert((values[i][0], values[i][1], values[i][2]));
    }
    println!("{:?}", surface);
    mp += 1;
    let start = (0, 0, 0);
    let mut current = VecDeque::new();

    current.push_front(start);
    let mut visit = HashSet::new();
    let mut edges = HashSet::new();
    while let Some(p) = current.pop_front() {
        for d in [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ] {
            let np = (p.0 + d.0, p.1 + d.1, p.2 + d.2);
            if np.0 < -1 || np.0 > mp || np.1 < -1 || np.1 > mp || np.2 < -1 || np.2 > mp {
                continue;
            }

            if position.contains(&np) {
                edges.insert((np, p));
            } else {
                if visit.insert(np) == false {
                    continue;
                }
                current.push_back(np);
            }
        }
    }
    println!("{}", edges.len());
}
