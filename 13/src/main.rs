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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Literal(i64),
}

fn parse(s: &mut VecDeque<char>) -> Item {
    let mut v = Vec::new();
    let mut val = String::new();
    while let Some(p) = s.pop_front() {
        if (p == ']' || p == ',') && val.len() > 0 {
            v.push(Item::Literal(parse_input(&val)));
            val = String::new();
        }
        if p == ']' {
            break;
        } else if p == '[' {
            v.push(parse(s));
        } else if p == ',' {
        } else {
            val += &p.to_string();
        }
    }
    Item::List(v)
}

impl Item {
    fn to_string(&self) -> String {
        match self {
            Item::List(p) => {
                let mut s = "[".to_owned();
                for (i, x) in p.iter().enumerate() {
                    if i > 0 {
                        s += ",";
                    }
                    s += &x.to_string();
                }

                s + "]"
            }
            Item::Literal(p) => p.to_string(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Item::Literal(p) => {
                if let Item::Literal(pp) = other {
                    return p.cmp(pp);
                }
                //self = literal, other = list
                return Item::List(vec![self.clone()]).cmp(other);
            }
            Item::List(p) => {
                if let Item::List(pp) = other {
                    for i in 0..(std::cmp::min(p.len(), pp.len())) {
                        let o = p[i].cmp(&pp[i]);
                        if o != std::cmp::Ordering::Equal {
                            return o;
                        }
                    }
                    return p.len().cmp(&pp.len());
                }
                return self.cmp(&Item::List(vec![other.clone()]));
            }
        }
        panic!()
    }
}

fn str_to_item(s: &str) -> Item {
    let mut pp1 = s.chars().collect::<VecDeque<_>>();
    pp1.pop_front();
    parse(&mut pp1)
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let mut p2 = Vec::new();
    for p in values {
        if p.len() == 0 {
            continue;
        }
        p2.push(str_to_item(&p));
    }
    let mut sum = 0;
    for (i, pp) in p2.chunks(2).enumerate() {
        if pp[0] < pp[1] {
            sum += i + 1;
        }
    }
    println!("{}", sum);
    let dp1 = str_to_item("[[2]]");
    let dp2 = str_to_item("[[6]]");
    p2.push(dp1.clone());
    p2.push(dp2.clone());

    p2.sort();
    let ind1 = p2.iter().position(|x| x == &dp1).unwrap() + 1;
    let ind2 = p2.iter().position(|x| x == &dp2).unwrap() + 1;
    println!("{} ", ind1 * ind2);
}
