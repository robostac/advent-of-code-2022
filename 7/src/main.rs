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

type EntryNode = Rc<RefCell<Entry>>;
#[derive(Debug, PartialEq, Eq, Clone)]
struct Entry {
    name: String,
    size: usize,
    contents: Vec<EntryNode>,
    dtype: EntryType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EntryType {
    FILE,
    FOLDER,
}

impl Entry {
    fn new(n: String, size: usize, t: EntryType) -> EntryNode {
        Rc::new(RefCell::new(Entry {
            name: n,
            size: size,
            contents: Vec::new(),
            dtype: t,
        }))
    }

    fn new_file(n: String, size: usize) -> EntryNode {
        return Self::new(n, size, EntryType::FILE);
    }
    fn new_folder(n: String) -> EntryNode {
        return Self::new(n, 0, EntryType::FOLDER);
    }

    fn size(&mut self) -> usize {
        if self.size > 0 {
            return self.size;
        }
        let mut sz = 0;
        for x in self.contents.iter_mut() {
            sz += x.borrow_mut().size();
        }
        self.size = sz;
        return sz;
    }

    fn dir_sizes(&mut self, sizes: &mut Vec<usize>) {
        if self.dtype == EntryType::FOLDER {
            sizes.push(self.size());
        }
        for x in self.contents.iter_mut() {
            x.borrow_mut().dir_sizes(sizes);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    let root = Entry::new_folder("/".to_owned());
    let mut cur_dir = root.clone();
    let mut dir_stack = Vec::new();
    let mut total_space = 70000000;
    for x in values {
        let line = x.split_ascii_whitespace().collect::<Vec<_>>();

        if line.len() == 0 {
            continue;
        }
        if line[0] == "$" {
            if line[1] == "ls" {
                continue;
            } else if line[1] == "cd" {
                if line[2] == "/" {
                    cur_dir = root.clone();
                    dir_stack.clear()
                } else if line[2] == ".." {
                    cur_dir = dir_stack.pop().unwrap();
                } else {
                    dir_stack.push(cur_dir.clone());

                    for p in dir_stack.last().unwrap().borrow().contents.iter() {
                        if p.borrow().name == line[2] {
                            cur_dir = p.clone();
                            break;
                        }
                    }
                }
            } else {
                panic!("{:?}", line);
            }
        } else {
            if line[0] == "dir" {
                cur_dir
                    .borrow_mut()
                    .contents
                    .push(Entry::new_folder(line[1].to_owned()));
            } else {
                let sz: usize = parse_input(line[0]);
                total_space -= sz;
                cur_dir
                    .borrow_mut()
                    .contents
                    .push(Entry::new_file(line[1].to_owned(), sz));
            }
        }
    }
    let mut sizes = Vec::new();
    root.borrow_mut().dir_sizes(&mut sizes);
    sizes.sort();

    println!(
        "{:?}",
        sizes.iter().filter(|x| **x <= 100000).sum::<usize>()
    );

    println!(
        "{:?}",
        sizes
            .iter()
            .filter(|x| (total_space + **x) >= 30000000)
            .next()
            .unwrap()
    );
}
