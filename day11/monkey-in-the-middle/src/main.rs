#![allow(dead_code)]
use std::fs;
fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

#[derive(Clone)]
struct Item {
    worry: i32,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: fn(i32) -> i32,
    test: fn(i32) -> bool,
    if_true: usize,
    if_false: usize,
    inspected: i32,
}

impl Monkey {
    fn from_block(block: &Vec<String>) -> Self {
        let starting_items: Vec<Item> = block
            .get(1)
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|s| Item {
                worry: s.parse::<i32>().unwrap(),
            })
            .collect();
        let operations: Vec<&str> = block.get(2).unwrap().split(": new = ").nth(1).unwrap().split_whitespace().collect();
        let number: i32 = operations.get(2).unwrap().parse::<i32>().unwrap();
        // let operation = match *operations.get(1).unwrap() {
        //     "*" => |old| old * number,
        //     "+" => |old| old + number,
        // }



    }

    fn throw(&self, index: usize, monkeys: &Vec<Monkey>) -> Vec<Monkey> {
        // return a new vector of monkeys where the throwing monkey lost the item and the receiving
        // monkey received it
        let inspected = self.inspected + 1;
        let item: Item = self.items.get(0).unwrap().clone();
        let new_worry_level = (self.operation)(item.worry);
        let test_outcome: bool = (self.test)(new_worry_level);
        let index_of_receiving_monkey: usize;
        match test_outcome {
            true => index_of_receiving_monkey = self.if_true,
            false => index_of_receiving_monkey = self.if_false,
        }
        let throwing_monkey = Self {
            items: self.items[1..].to_owned(),
            inspected,
            ..*self
        };
        let receiving_monkey = monkeys.get(index_of_receiving_monkey).unwrap();
        let mut new_items = receiving_monkey.items.to_owned();
        new_items.push(item);
        let receiving_monkey = Self {
            items: new_items,
            ..*receiving_monkey
        };
        monkeys
            .to_owned()
            .iter()
            .enumerate()
            .map(|(i, monkey)| match i {
                a if a == index => throwing_monkey.clone(),
                a if a == index_of_receiving_monkey => receiving_monkey.clone(),
                _ => monkey.clone(),
            })
            .collect()
    }
}

#[derive(Clone)]
struct State {
    monkeys: Vec<Monkey>,
}

impl State {
    fn next(&self, monkey_index: usize) -> State {
        let throwing_monkey = self.monkeys.get(monkey_index).unwrap();
        let new_monkeys = throwing_monkey.throw(monkey_index, &self.monkeys);
        State {
            monkeys: new_monkeys,
        }
    }
}

struct Round {
    state: State,
    round: i32,
}

impl Round {
    fn next(&self) -> Round {
        let mut next_state: State = self.state.to_owned();
        for monkey_index in 0..self.state.monkeys.len() {
            for _item in &self.state.monkeys.get(monkey_index).unwrap().items {
                next_state = next_state.next(monkey_index);
            }
        }
        Round {
            state: next_state,
            round: self.round + 1,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
