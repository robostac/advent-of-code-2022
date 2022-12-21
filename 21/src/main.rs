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

#[derive(Debug, Clone)]
struct Node {
    left: usize,
    right: usize,
    v: i64,
}

impl Node {
    fn new(x: i64) -> Node {
        Node {
            left: 0,
            right: 0,
            v: x,
        }
    }
}

fn solve(mut dq: VecDeque<(String, Vec<String>)>, mut calc: HashMap<String, i64>) -> i64 {
    while let Some((a, b)) = dq.pop_front() {
        if let Some(v1) = calc.get(&b[0]) {
            if let Some(v2) = calc.get(&b[2]) {
                let v = match b[1].chars().next().unwrap() {
                    '+' => v1 + v2,
                    '-' => v1 - v2,
                    '/' => v1 / v2,
                    '*' => v1 * v2,
                    _ => panic!(),
                };
                calc.insert(a, v);
                continue;
            }
        }
        dq.push_back((a, b));
    }
    return calc["root"];
}

fn solvep2(
    mut dq: VecDeque<(String, Vec<String>)>,
    mut calc: HashMap<String, i64>,
    hmn: i64,
) -> (i64, i64) {
    calc.insert("humn".to_owned(), hmn);
    while let Some((a, b)) = dq.pop_front() {
        if a == "humn" {
            continue;
        }
        if let Some(v1) = calc.get(&b[0]) {
            if let Some(v2) = calc.get(&b[2]) {
                if a == "root" {
                    return (*v1, *v2);
                }
                let v = match b[1].chars().next().unwrap() {
                    '+' => v1.saturating_add(*v2),
                    '-' => v1.saturating_sub(*v2),
                    '/' => v1.saturating_div(*v2),
                    '*' => v1.saturating_mul(*v2),
                    _ => panic!(),
                };
                calc.insert(a, v);
                continue;
            }
        }
        dq.push_back((a, b));
    }
    return (0, 0);
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

    let mut dq = VecDeque::new();
    let mut calc = HashMap::new();
    for x in values {
        let (a, b) = x.split_once(": ").unwrap();
        if let Result::Ok(v) = b.parse::<i64>() {
            calc.insert(a.to_owned(), v);
        } else {
            dq.push_back((
                a.to_owned(),
                b.split_ascii_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<_>>(),
            ));
        }
    }

    println!("{:?}", solve(dq.clone(), calc.clone()));

    let ansp2 = binary_search_range_max(i64::MIN, i64::MAX, &|x| {
        let (a, b) = solvep2(dq.clone(), calc.clone(), x);
        a < b
    }) + 1;

    let (a, b) = solvep2(dq.clone(), calc.clone(), ansp2);

    if a != b {
        let ansp2 = binary_search_range_max(i64::MIN, i64::MAX, &|x| {
            let (a, b) = solvep2(dq.clone(), calc.clone(), x);
            a > b
        }) + 1;
        let (a, b) = solvep2(dq.clone(), calc.clone(), ansp2);
        println!("{} {:?}", ansp2, (a, b));
    } else {
        println!("{} {:?}", ansp2, (a, b));
    }
}
