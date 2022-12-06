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

fn find_cont(s: &str, count: usize) -> Option<usize> {
    let mut current = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        current.push_back(c);
        if current.len() == count {
            if current.iter().collect::<HashSet<_>>().len() == count {
                return Some(i + 1);
            }
            current.pop_front();
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

    if let Some(p1) = find_cont(&values[0], 4) {
        println!("P1: {}", p1);
    }
    if let Some(p2) = find_cont(&values[0], 14) {
        println!("P2: {}", p2);
    }
}
