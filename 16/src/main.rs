#![allow(dead_code, unused_macros, unused_imports)]

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
    input.trim().parse().unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Valve {
    rate: i64,
    children: Vec<usize>,
}

fn get_index(x: &str, hm: &mut HashMap<String, usize>) -> usize {
    if let Some(p) = hm.get(x) {
        return *p;
    }
    let e = hm.entry("next".to_owned()).or_insert(0);
    let idx = *e;
    *e += 1;
    hm.insert(x.to_owned(), idx);
    idx
}

fn dfs(
    valves: &Vec<Valve>,
    mask: usize,
    time: usize,
    dist: &Vec<Vec<usize>>,
    pos: usize,
    good: &Vec<usize>,
    cur_time: usize,
) -> i64 {
    let mut best = 0;
    for p in good.iter() {
        if (1 << *p) & mask > 0 {
            continue;
        }
        let d = dist[pos][*p] + 1;
        let next_time = cur_time + d;
        if next_time >= time {
            continue;
        }
        let v = dfs(valves, mask | 1 << *p, time, dist, *p, good, next_time);
        let open = (time - next_time) as i64 * valves[*p].rate;
        best = std::cmp::max(v + open, best);
    }

    best
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();
    let mut name_map = HashMap::new();

    let mut valves: Vec<Valve> = vec![Default::default(); values.len()];
    for x in values {
        let x = x.replace(",", "");
        let (details, children) = x.split_once(";").unwrap();
        let (_, rate) = details.split_once("=").unwrap();
        let rate = parse_input::<i64>(rate);
        let it = details.split_ascii_whitespace();
        let name = it.skip(1).next().unwrap();

        let it = children.split_ascii_whitespace();

        let mut cc = Vec::new();
        for y in it.skip(4) {
            let cidx = get_index(y, &mut name_map);
            cc.push(cidx);
        }
        let idx = get_index(name, &mut name_map);
        valves[idx].children = cc;
        valves[idx].rate = rate;
    }

    let mut dist = vec![vec![2000; valves.len()]; valves.len()];
    for (i, v1) in valves.iter().enumerate() {
        dist[i][i] = 0;
        for x in v1.children.iter() {
            dist[i][*x] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    let start = name_map["AA"];

    let mut good_valves = Vec::new();
    for (i, p) in valves.iter().enumerate() {
        if p.rate > 0 {
            good_valves.push(i);
        }
    }
    println!("{}", dfs(&valves, 0, 30, &dist, start, &good_valves, 0));

    let mut bv = 0;
    for m in 0..=(1 << good_valves.len()) {
        let mut man_mask = 0;
        let mut ele_mask = 0;
        for (i, x) in good_valves.iter().enumerate() {
            if (m & (1 << i)) == 0 {
                man_mask |= 1 << *x;
            } else {
                ele_mask |= 1 << *x;
            }
        }
        let v = dfs(&valves, man_mask, 26, &dist, start, &good_valves, 0)
            + dfs(&valves, ele_mask, 26, &dist, start, &good_valves, 0);
        bv = std::cmp::max(bv, v);
    }
    println!("{}", bv);
    // for i in 0..30 {
    //     let mut v = HashMap::new();
    //     std::mem::swap(&mut v, &mut current[i]);
    //     for ((pos, epos, mask, released), releasing) in v {
    //         best = std::cmp::max(best, released + (current.len() - i) as i64 * releasing);
    //         for p in 0..valves.len() {
    //             if valves[p].rate == 0 {
    //                 continue;
    //             }
    //             if (mask & (1 << p)) > 0 {
    //                 continue;
    //             }
    //             for p2 in 0..valves.len() {
    //                 if p2 == p {
    //                     continue;
    //                 }
    //                 if valves[p2].rate == 0 {
    //                     continue;
    //                 }
    //                 if (mask & (1 << p2)) > 0 {
    //                     continue;
    //                 }
    //                 let d = dist[pos][p] + 1;
    //                 let next_time = i + d;
    //                 if next_time >= current.len() {
    //                     continue;
    //                 }
    //                 let k = (p, mask | 1 << p, released + (d as i64 * releasing));
    //                 let e = current[next_time].entry(k).or_insert(0);
    //                 let tgt = releasing + valves[p].rate;
    //                 if tgt > *e {
    //                     *e = tgt;
    //                 }
    //             }
    //         }
    //     }
    //     println!("best {} {}", i + 1, best);
    // }
}
