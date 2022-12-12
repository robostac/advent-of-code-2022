#![allow(dead_code, unused_macros, unused_imports)]

use std::cell::RefCell;
use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

fn parse_input<Type>(input: &str) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
{
    input.trim().parse().unwrap()
}
#[derive(Clone, Debug)]
enum MonkeyValue {
    Literal(i64),
    Old(),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    divis: i64,
    op1: MonkeyValue,
    op2: MonkeyValue,
    mult: bool,
    target_true: usize,
    target_false: usize,
    excount: i64,
}

fn parse_items(s: &str) -> Vec<i64> {
    let (_, b) = s.split_once(':').unwrap();
    let b = b
        .split(',')
        .map(|x| parse_input::<i64>(x))
        .collect::<Vec<_>>();
    b
}

fn parse_op(s: &str) -> (MonkeyValue, MonkeyValue, bool) {
    let (_, s) = s.split_once('=').unwrap();
    let divis;
    let v2;
    if let Some((_, s)) = s.split_once('*') {
        v2 = s;
        divis = true;
    } else if let Some((_, s)) = s.split_once('+') {
        v2 = s;
        divis = false;
    } else {
        panic!()
    }
    let op2;
    if v2.trim() == "old" {
        op2 = MonkeyValue::Old();
    } else {
        op2 = MonkeyValue::Literal(parse_input(v2));
    }
    (MonkeyValue::Old(), op2, divis)
}

fn parse_divis(s: &str) -> i64 {
    let b = s.split(' ').last().unwrap();
    parse_input(b)
}

fn parse_target(s: &str) -> usize {
    let b = s.split(' ').last().unwrap();
    parse_input(b)
}
impl Monkey {
    fn new(s: &[String]) -> Self {
        let items = parse_items(&s[1]);
        let (op1, op2, mult) = parse_op(&s[2]);
        let divis = parse_divis(&s[3]);
        let target_true = parse_target(&s[4]);
        let target_false = parse_target(&s[5]);

        Monkey {
            items,
            divis,
            op1,
            op2,
            mult,
            target_true,
            target_false,
            excount: 0,
        }
    }

    fn examine(&mut self, a: i64) -> i64 {
        let v2 = match self.op2 {
            MonkeyValue::Old() => a,
            MonkeyValue::Literal(p) => p,
        };
        let mut v = a;
        if self.mult {
            v *= v2;
        } else {
            v += v2;
        }
        self.excount += 1;
        v
    }
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn solve(mut monkeys: Vec<Monkey>, divis: i64, rounds: usize, lcm: i64) -> i64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut t = Vec::new();
            std::mem::swap(&mut monkeys[i].items, &mut t);
            for p in t {
                let vv = (monkeys[i].examine(p) / divis) % lcm;
                let tgt;
                if vv % monkeys[i].divis == 0 {
                    tgt = monkeys[i].target_true;
                } else {
                    tgt = monkeys[i].target_false;
                }
                monkeys[tgt].items.push(vv);
            }
        }
    }
    let mut d = monkeys.iter().map(|x| x.excount).collect::<Vec<_>>();
    d.sort();
    d[monkeys.len() - 1] * d[monkeys.len() - 2]
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut monkeys = Vec::new();
    for x in values.chunks(7) {
        monkeys.push(Monkey::new(x));
    }
    let mut monkeylcm = monkeys[0].divis;
    for x in monkeys.iter() {
        monkeylcm = lcm(monkeylcm, x.divis);
    }

    println!("{}", solve(monkeys.clone(), 3, 20, monkeylcm));
    println!("{}", solve(monkeys.clone(), 1, 10000, monkeylcm));
}
