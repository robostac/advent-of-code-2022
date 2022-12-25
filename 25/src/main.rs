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

fn to_snafu(s: i64) -> String {
    let mut ss = VecDeque::new();
    let mut s = s;
    let mut carry = 0;
    while s > 0 {
        let c = s % 5;
        s /= 5;
        ss.push_back(c);
    }
    let mut sss = String::new();

    while ss.len() > 0 || carry != 0 {
        let mut a = carry;
        if let Some(p) = ss.pop_front() {
            a += p;
        }
        carry = 0;
        if a > 2 {
            a = a - 5;
            carry = 1;
        }
        sss += &match a {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        }
        .to_string();
    }
    // println!("{:?}", sss);
    sss.chars().rev().collect::<String>()
}

fn from_snafu(s: &str) -> i64 {
    let mut x = 0;
    for p in s.chars() {
        x *= 5;
        x += match p {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
    }
    x
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut ans = 0;
    for p in values {
        // println!("{} {} {}", p, from_snafu(&p), to_snafu(from_snafu(&p)));
        ans += from_snafu(&p);
    }
    println!("{:?}", ans);
    println!("{:?}", to_snafu(ans));
}
