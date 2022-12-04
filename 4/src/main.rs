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

fn parse(x: &str) -> (i64, i64) {
    let z = x
        .split("-")
        .map(|x| parse_input::<i64>(x))
        .collect::<Vec<_>>();
    return (z[0], z[1]);
}

fn includes(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn overlaps(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && a.1 >= b.0
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut p1score = 0;
    let mut p2score = 0;
    for p in values {
        let z = p.split(",").map(|x| parse(x)).collect::<Vec<_>>();

        if includes(z[0], z[1]) {
            p1score += 1;
        } else if includes(z[1], z[0]) {
            p1score += 1;
        }
        if overlaps(z[0], z[1]) {
            p2score += 1;
        } else if overlaps(z[1], z[0]) {
            p2score += 1;
        }
    }
    println!("{:?}", p1score);
    println!("{:?}", p2score);
}
