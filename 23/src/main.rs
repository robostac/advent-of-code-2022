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

const DIRECTIONS: [(i64, i64, u8); 8] = [
    (0, -1, 0),
    (1, -1, 1),
    (1, 0, 2),
    (1, 1, 3),
    (0, 1, 4),
    (-1, 1, 5),
    (-1, 0, 6),
    (-1, -1, 7),
];

const MOVES: [(u8, (i64, i64)); 4] = [
    (0b10000011, (0, -1)),
    (0b00111000, (0, 1)),
    (0b11100000, (-1, 0)),
    (0b00001110, (1, 0)),
];

fn adj_mask(p: (i64, i64), g: &HashSet<(i64, i64)>) -> u8 {
    let mut m = 0;
    for d in DIRECTIONS {
        let np = (p.0 + d.0, p.1 + d.1);
        if g.contains(&np) {
            m |= 1 << d.2;
        }
    }
    m
}

fn move_elves(elfpos: &Vec<(i64, i64)>, mi: usize) -> Vec<(i64, i64)> {
    let mut elfprop = elfpos.clone();
    let mut pos_map = HashMap::new();
    let current = elfpos.iter().cloned().collect::<HashSet<_>>();
    let elf_adj = elfpos
        .iter()
        .map(|x| adj_mask(*x, &current))
        .collect::<Vec<_>>();
    for i in 0..elfpos.len() {
        if elf_adj[i] != 0 {
            for z in 0..MOVES.len() {
                let idx = (z + mi) % MOVES.len();
                if (elf_adj[i] & MOVES[idx].0) == 0 {
                    elfprop[i] = (elfpos[i].0 + MOVES[idx].1 .0, elfpos[i].1 + MOVES[idx].1 .1);
                    *pos_map.entry(elfprop[i]).or_insert(0) += 1;
                    break;
                }
            }
        }
    }
    for i in 0..elfpos.len() {
        if let Some(v) = pos_map.get(&elfprop[i]) {
            if *v > 1 {
                elfprop[i] = elfpos[i];
            }
        }
    }

    elfprop
}

fn print_elf_pos(ep: &Vec<(i64, i64)>) {
    let current = ep.iter().cloned().collect::<HashSet<_>>();
    let minx = ep.iter().map(|x| x.0).min().unwrap();
    let maxx = ep.iter().map(|x| x.0).max().unwrap();
    let miny = ep.iter().map(|x| x.1).min().unwrap();
    let maxy = ep.iter().map(|x| x.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if current.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut elfpos = Vec::new();
    for (y, l) in values.iter().enumerate() {
        for (x, v) in l.chars().enumerate() {
            if v == '#' {
                elfpos.push((x as i64, y as i64));
            }
        }
    }
    // print_elf_pos(&elfpos);
    for i in 0..10 {
        elfpos = move_elves(&elfpos, i);
        // print_elf_pos(&elfpos);
    }
    let minx = elfpos.iter().map(|x| x.0).min().unwrap();
    let maxx = elfpos.iter().map(|x| x.0).max().unwrap();
    let miny = elfpos.iter().map(|x| x.1).min().unwrap();
    let maxy = elfpos.iter().map(|x| x.1).max().unwrap();

    let sqsize = (maxx - minx + 1) * (maxy - miny + 1);
    println!("{:?}", sqsize - elfpos.len() as i64);
    for i in 10.. {
        let ep = move_elves(&elfpos, i);
        if ep == elfpos {
            println!("{}", i + 1);
            break;
        }
        elfpos = ep;
    }
}
