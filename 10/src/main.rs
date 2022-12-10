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
#[derive(Debug, PartialEq, Eq, Clone)]
enum Inst {
    Noop,
    Addx(i64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Circuit {
    x: i64,
    cycle: i64,
    inst: Vec<Inst>,
    history: Vec<(i64, i64)>,
}

impl Circuit {
    fn new(inst: Vec<Inst>) -> Self {
        Circuit {
            x: 1,
            cycle: 1,
            inst: inst,
            history: Vec::new(),
        }
    }

    fn execute(&mut self) {
        self.cycle = 1;
        for x in self.inst.iter() {
            match x {
                Inst::Noop => {
                    self.cycle += 1;
                }
                Inst::Addx(p) => {
                    self.cycle += 2;
                    self.x += p;
                    self.history.push((self.cycle, self.x));
                }
            }
        }
    }
}

fn binary_search_range_max(l: i64, r: i64, res: &dyn Fn(i64) -> bool) -> i64 {
    let mut l = l;
    let mut r = r;
    if res(r) {
        return r;
    }
    while (l + 1) < r {
        let m = l.saturating_add(r) / 2;
        let v = res(m);
        if v == true {
            l = m;
        } else {
            r = m;
        }
    }
    return l;
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut inst = Vec::new();

    for x in values {
        if x == "noop" {
            inst.push(Inst::Noop);
        } else {
            let (a, b) = x.split_once(' ').unwrap();
            let b = parse_input::<i64>(b);
            inst.push(Inst::Addx(b));
        }
    }
    let mut cr = Circuit::new(inst);
    cr.execute();
    let mut s = 0;
    for p in (20..=220).step_by(40) {
        let z = binary_search_range_max(0, cr.history.len() as i64 - 1, &|x| {
            cr.history[x as usize].0 <= p
        });
        s += cr.history[z as usize].1 * p;
    }
    println!("{:?}", s);

    let mut p = vec![Vec::new(); 6];
    let mut c = 1;
    let mut curx = 1;
    let mut idx = 0;
    for y in 0..6 {
        for x in 1..=40 {
            if idx < cr.history.len() && cr.history[idx].0 <= c {
                curx = cr.history[idx].1;
                idx += 1;
            }
            let v = x - curx;
            if v >= 0 && v <= 2 {
                p[y].push('#');
            } else {
                p[y].push('.');
            }

            c += 1;
        }
    }
    for x in p {
        for i in x {
            print!("{}", i);
        }
        println!();
    }
}
