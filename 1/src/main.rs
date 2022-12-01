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

fn part_1(s: &Vec<i64>) -> i64 {
    s[0]
}

fn part_2(s: &Vec<i64>) -> i64 {
    s[..3].iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut elfs = Vec::new();
    let mut cur_elf: i64 = 0;
    elfs.push(0);
    for x in values.iter() {
        if x.len() == 0 {
            elfs.push(0);
        } else {
            *elfs.iter_mut().last().unwrap() += parse_input::<i64>(x);
        }
    }
    elfs.sort();
    elfs.reverse();
    println!("{:?}", part_1(&elfs));
    println!("{:?}", part_2(&elfs));
}
