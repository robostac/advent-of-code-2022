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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct State {
    resources: [i64; 4],
    robots: [i64; 4],
}

fn dfs(
    bp: &Vec<Vec<i64>>,
    p: &State,
    time: usize,
    ct: usize,
    more: &Vec<i64>,
    cur_best: &mut i64,
) -> i64 {
    let rem = (time - ct) as i64;
    let wait = p.resources[3] + p.robots[3] * rem;
    *cur_best = std::cmp::max(wait, *cur_best);
    let bb = wait + (rem * (rem + 1)) / 2;
    if bb <= *cur_best {
        return 0;
    }
    for (i, bp_req) in bp.iter().enumerate().rev() {
        if p.robots[i] >= more[i] {
            continue;
        }
        let mut days = 0;
        for j in 0..3 {
            if bp_req[j] > 0 && p.robots[j] == 0 {
                days = 100;
                break;
            }
            let extra = bp_req[j] - p.resources[j];
            if extra > 0 {
                days = std::cmp::max(days, (extra + p.robots[j] - 1) / p.robots[j]);
            }
        }
        let days = 1 + days as usize;
        if ct + days < time {
            let mut np = p.clone();
            for j in 0..4 {
                np.resources[j] += days as i64 * np.robots[j];
            }
            for j in 0..3 {
                np.resources[j] -= bp_req[j];
            }
            np.robots[i] += 1;

            *cur_best = std::cmp::max(dfs(bp, &np, time, ct + days, more, cur_best), *cur_best);
        }
    }
    *cur_best
}

fn check(bp: &Vec<Vec<i64>>, time: usize) -> i64 {
    let mut start: State = Default::default();
    start.robots[0] = 1;
    let mut more = vec![0, 0, 0, time as i64];
    for x in bp.iter() {
        for i in 0..3 {
            more[i] = std::cmp::max(more[i], x[i]);
        }
    }

    let v = dfs(bp, &start, time, 0, &more, &mut 0);

    v
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let mut bp = Vec::new();
    for x in values {
        let s = x.split_ascii_whitespace().collect::<Vec<_>>();

        let ore: i64 = parse_input(&s[6]);
        let clay: i64 = parse_input(&s[12]);
        let obs_ore: i64 = parse_input(&s[18]);
        let obs_clay: i64 = parse_input(&s[21]);
        let geode_ore: i64 = parse_input(&s[27]);
        let geode_obs: i64 = parse_input(&s[30]);
        bp.push(vec![
            vec![ore, 0, 0],
            vec![clay, 0, 0],
            vec![obs_ore, obs_clay, 0],
            vec![geode_ore, 0, geode_obs],
        ]);
    }
    let mut ans = 0;

    for (i, x) in bp.iter().enumerate() {
        let v = check(x, 24);
        ans += (i as i64 + 1) * v;
    }
    println!("{}", ans);

    let mut p2ans = 1;
    while bp.len() > 3 {
        bp.pop();
    }
    for (i, x) in bp.iter().enumerate() {
        let v = check(x, 32);
        p2ans *= v;
    }
    println!("{}", p2ans);
}
