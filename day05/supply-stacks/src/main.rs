use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(input: &String) -> Self {
        let move_elements: Vec<usize> = input
            .split(" ")
            .enumerate()
            .filter(|(i, _)| vec![1, 3, 5].contains(i))
            .map(|(_, c)| c.parse::<usize>().unwrap())
            .collect();
        Move {
            amount: move_elements[0],
            from: move_elements[1],
            to: move_elements[2],
        }
    }

    fn apply(&self, stacks: &mut HashMap<usize, VecDeque<char>>) {
        for _ in 0..self.amount {
            let stack: &mut VecDeque<char> = stacks.get_mut(&self.from).unwrap();
            let top_crate: char = stack.pop_front().unwrap();
            let stack: &mut VecDeque<char> = stacks.get_mut(&self.to).unwrap();
            stack.insert(0, top_crate);
        }
    }


    fn apply_9001(&self, stacks: &mut HashMap<usize, VecDeque<char>>) {
        let stack: &mut VecDeque<char> = stacks.get_mut(&self.from).unwrap();
        let mut top_crates: VecDeque<char> = VecDeque::new();
        for _ in (0..self.amount) {
            top_crates.insert(0, stack.pop_front().unwrap());
        }
        let stack: &mut VecDeque<char> = stacks.get_mut(&self.to).unwrap();
        for c in top_crates.iter() {
            stack.insert(0, *c);
        }
    }
}

fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn split_input(input: &Vec<String>) -> (&[String], Vec<i32>, &[String]) {
    let blank_line_index = input.iter().position(|x| x == "").unwrap();
    let crate_labels = &input[..blank_line_index - 1];
    let crate_positions: Vec<i32> = input[blank_line_index - 1]
        .split(" ")
        .filter(|&s| s != "")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let moves = &input[blank_line_index + 1..];
    (crate_labels, crate_positions, moves)
}

fn parse_crate_labels(input: &[String]) -> HashMap<usize, VecDeque<char>> {
    let crate_tuples: Vec<Vec<(usize, char)>> = input
        .iter()
        .map(|s| {
            s.chars()
                .enumerate()
                .filter(|(i, _)| (i % 4) == 1)
                .filter(|(_, c)| c != &' ')
                .map(|(i, c)| ((i / 4) + 1, c))
                .collect()
        })
        .collect();
    let crate_tuples_flattened: Vec<(usize, char)> =
        crate_tuples.iter().rev().fold(Vec::new(), |mut acc, v| {
            for t in v {
                acc.push(*t);
            }
            acc
        });
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    for (i, c) in crate_tuples_flattened {
        let stack = stacks.entry(i).or_insert(VecDeque::new());
        stack.insert(0, c);
    }
    stacks
}

fn get_top_crates(stacks: &HashMap<usize, VecDeque<char>>) -> Vec<&char> {
    let mut top_crates: Vec<&char> = Vec::new();
    for i in (1..10).rev() {
        top_crates.insert(0, stacks.get(&i).unwrap().get(0).unwrap());
    }
    top_crates
}

fn parse_moves(input: &[String]) -> Vec<Move> {
    input.iter().map(|l| Move::from_line(&l)).collect()
}

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let (crate_labels, crate_positions, moves) = split_input(&input);
    let mut stacks = parse_crate_labels(crate_labels);
    let moves = parse_moves(moves);
    for mv in moves.iter() {
        mv.apply(&mut stacks);
    }
    println!("{:?}", stacks);
    let top_crates = get_top_crates(&stacks);
    println!("{:?}", top_crates);
}

fn part_two() {
    let input = read_input("./input.txt", "\n");
    let (crate_labels, crate_positions, moves) = split_input(&input);
    let mut stacks = parse_crate_labels(crate_labels);
    let moves = parse_moves(moves);
    for mv in moves.iter() {
        mv.apply_9001(&mut stacks);
    }
    let top_crates = get_top_crates(&stacks);
    println!("{:?}", top_crates);
}

fn main() {
    println!("Hello, world!");
    // part_one();
    part_two();
}
