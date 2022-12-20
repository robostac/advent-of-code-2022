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

fn remove_node(nodes: &mut Vec<Node>, idx: usize) {
    let l = nodes[idx].left;
    let r = nodes[idx].right;
    nodes[l].right = r;
    nodes[r].left = l;
}

fn insert_node_right(nodes: &mut Vec<Node>, idx: usize, insert_pos: usize) {
    let r = nodes[insert_pos].right;
    nodes[insert_pos].right = idx;
    nodes[idx].left = insert_pos;
    nodes[idx].right = r;
    nodes[r].left = idx;
}

fn move_node(nodes: &mut Vec<Node>, idx: usize, p: i64) {
    remove_node(nodes, idx);
    let mut p = p % (nodes.len() as i64 - 1);
    if p < 0 {
        p = nodes.len() as i64 - 1 + p;
    }
    let dist = p as usize % (nodes.len() - 1);
    let insert_pos;
    let mut r = nodes[idx].right;
    for _ in 0..dist {
        r = nodes[r].right;
    }
    insert_pos = nodes[r].left;
    insert_node_right(nodes, idx, insert_pos);
}

fn solve(values: &[i64], loops: usize) -> i64 {
    let mut nodes = Vec::new();
    for i in 0..values.len() {
        let mut n = Node::new(values[i]);
        n.right = (i + 1) % values.len();
        n.left = (i + values.len() - 1) % values.len();
        nodes.push(n);
    }

    for _ in 0..loops {
        for (i, p) in values.iter().enumerate() {
            move_node(&mut nodes, i, *p);
        }
    }
    let zero_idx = values.iter().position(|x| *x == 0).unwrap();
    let mut idx = zero_idx;
    let mut c = Vec::new();
    for _ in 0..3 {
        for _ in 0..1000 {
            idx = nodes[idx].right;
        }
        c.push(nodes[idx].v);
    }
    c.iter().sum::<i64>()
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    println!("{}", solve(&values, 1));

    let p2values = values.iter().map(|x| *x * 811589153).collect::<Vec<_>>();
    println!("{}", solve(&p2values, 10));
}
