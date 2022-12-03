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

fn prio(p: char) -> i32 {
    if p >= 'a' && p <= 'z' {
        return (p as i32 - 'a' as i32) + 1;
    } else {
        return (p as i32 - 'A' as i32) + 27;
    }
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
    let mut p2count = 0;
    let mut p2set = HashSet::new();
    for x in values.iter() {
        let a = &x[..(x.len() / 2)];
        let b = &x[(x.len() / 2)..];
        let aa = a.chars().collect::<HashSet<_>>();
        let bb = b.chars().collect::<HashSet<_>>();
        let z = aa.intersection(&bb);
        for p in z {
            p1score += prio(*p);
        }
        let fullset = x.chars().collect::<HashSet<_>>();
        if p2count == 0 {
            p2set = fullset
        } else {
            p2set.retain(|x| fullset.contains(x));
        }
        p2count += 1;
        if p2count == 3 {
            for p in p2set.iter() {
                p2score += prio(*p);
            }
            p2count = 0;
        }
    }
    println!("{}", p1score);
    println!("{}", p2score);
}
