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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn new(s: &str) -> Self {
        if s == "A" || s == "X" {
            return RPS::Rock;
        } else if s == "B" || s == "Y" {
            return RPS::Paper;
        } else if s == "C" || s == "Z" {
            return RPS::Scissors;
        }
        panic!()
    }

    fn score(&self) -> i64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn beats(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn loses(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}
impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if *self == *other {
            return std::cmp::Ordering::Equal;
        }
        let o = self.beats();
        if *other == o {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Less;
    }
}

fn part1(r: &Vec<(RPS, RPS)>) -> i64 {
    let mut score = 0;
    for (a, b) in r.iter() {
        let s1 = match b.cmp(a) {
            std::cmp::Ordering::Greater => 6,
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
        };

        score += b.score() + s1;
        // println!("{:?} {:?} {:?} {:?}", 0, b, a, b.cmp(a));
    }
    score
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut rounds = Vec::new();
    let mut rounds2 = Vec::new();
    for x in values {
        let mut d = x.split(" ");
        let a = RPS::new(d.next().unwrap());
        let z = d.next().unwrap();
        let b = RPS::new(z);

        rounds.push((a, b));
        if z == "X" {
            rounds2.push((a, a.beats()));
        } else if z == "Y" {
            rounds2.push((a, a));
        } else {
            rounds2.push((a, a.loses()));
        }
    }

    println!("{:?}", part1(&rounds));
    println!("{:?}", part1(&rounds2));
}
