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

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut stack_input = Vec::new();
    let mut inst = Vec::new();
    let mut first_part = true;
    for p in values {
        if p.len() == 0 {
            first_part = false;
        } else if first_part {
            stack_input.push(p);
        } else {
            inst.push(p);
        }
    }
    let count_str = stack_input.pop().unwrap();
    let mut stacks = Vec::new();
    stacks.push(Vec::new()); // 0
    for (i, p) in count_str.chars().enumerate() {
        if p == ' ' {
            continue;
        }
        let mut v = Vec::new();
        for z in stack_input.iter().rev() {
            if let Some(p) = z.chars().nth(i) {
                if p == ' ' {
                    break;
                }
                v.push(p);
            }
        }
        stacks.push(v);
    }

    let mut p2stacks = stacks.clone();
    for p in inst {
        let z = p.split(" ").collect::<Vec<_>>();
        let count: usize = parse_input(z[1]);
        let src: usize = parse_input(z[3]);
        let dest: usize = parse_input(z[5]);
        let mut removed = Vec::new();
        for _ in 0..count {
            let p = stacks[src].pop().unwrap();
            stacks[dest].push(p);
            let p = p2stacks[src].pop().unwrap();
            removed.push(p);
        }
        p2stacks[dest].extend(removed.iter().rev());
    }
    for x in stacks {
        if x.len() > 0 {
            print!("{}", x.iter().last().unwrap());
        }
    }
    println!();
    for x in p2stacks {
        if x.len() > 0 {
            print!("{}", x.iter().last().unwrap());
        }
    }
    println!();
    // println!("{:?}", stacks);
}
