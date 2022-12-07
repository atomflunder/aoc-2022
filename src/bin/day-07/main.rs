// Day 7 of Advent of Code 2022.
// https://adventofcode.com/2022/day/7

use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Node {
    pub size: Option<usize>,
    // True if it is a file, false if it is a directory.
    pub file_or_dir: bool,
    pub child_node: HashMap<String, Rc<RefCell<Node>>>,
    pub parent_node: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            size: None,
            file_or_dir: false,
            child_node: HashMap::new(),
            parent_node: None,
        }
    }
}

fn input_to_nodes(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut cur_node = Rc::clone(&root);

    for line in input.lines() {
        let commands: Vec<&str> = line.split(' ').collect();

        // If it is a command we check if it is the cd command and where we cd into.
        if line.starts_with('$') {
            if line.contains("cd") {
                let folder = commands[2];
                cur_node = match folder {
                    // The node directly above it.
                    ".." => Rc::clone(cur_node.borrow().parent_node.as_ref().unwrap()),
                    // The root node. Will only come up in the first command.
                    "/" => root.clone(),
                    // Everything else will be us cding into a deeper node.
                    _ => cur_node.borrow().child_node.get(folder).unwrap().clone(),
                }
            }
        // Otherwise, these will be the files or directories that will be listed with their name.
        } else if let [size_or_dir, name] = &commands[..] {
            if !cur_node.borrow().child_node.contains_key(*name) {
                let child = Rc::new(RefCell::new(Node::new()));
                let mut mut_child = child.borrow_mut();
                // This can either be "dir" or a number indicating the file size.
                if *size_or_dir != "dir" {
                    mut_child.file_or_dir = true;
                    mut_child.size = Some(size_or_dir.parse().unwrap());
                }

                mut_child.parent_node = Some(Rc::clone(&cur_node));
                cur_node
                    .borrow_mut()
                    .child_node
                    .insert(name.to_string(), Rc::clone(&child));
            }
        }
    }
    root
}

fn calculate_sum<'a>(node: &'a Node, sizes: &'a mut Vec<usize>) -> (usize, &'a mut Vec<usize>) {
    if node.file_or_dir {
        return (node.size.unwrap(), sizes);
    }

    let size = node
        .child_node
        .values()
        .map(|child| calculate_sum(&child.borrow(), sizes).0)
        .sum();
    sizes.push(size);
    (size, sizes)
}

fn part_one() {
    let root_node = input_to_nodes(include_str!("commands.txt"));
    let mut size_start = vec![];
    let binding = root_node.borrow();
    let (_, sizes) = calculate_sum(&binding, &mut size_start);
    // Only add up the files with less than 100_000 in size.
    println!("{}", sizes.iter().filter(|x| **x < 100_000).sum::<usize>());
}

fn part_two() {
    let root = input_to_nodes(include_str!("commands.txt"));
    let mut size_start = vec![];
    let binding = root.borrow();
    let (cur_used, sizes) = calculate_sum(&binding, &mut size_start);
    let needed = 30_000_000 - (70_000_000 - cur_used);
    println!("{}", *sizes.iter().filter(|x| **x > needed).min().unwrap());
}

fn main() {
    part_one();
    part_two();
}
