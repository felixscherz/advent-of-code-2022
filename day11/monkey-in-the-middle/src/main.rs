#![allow(dead_code)]
use std::fs;
fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

#[derive(Clone, Debug)]
struct Item {
    worry: u64,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Mul(Value),
    Add(Value),
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
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
            .split(", ")
            .map(|s| Item {
                worry: s.parse::<u64>().unwrap(),
            })
            .collect();
        let operations: Vec<&str> = block
            .get(2)
            .unwrap()
            .split(": new = ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect();
        let number_or_old: _ = match *operations.get(2).unwrap() {
            "old" => Value::Old,
            a => Value::Num(a.parse::<u64>().unwrap()),
        };
        let op: Operation = match *operations.get(1).unwrap() {
            "*" => Operation::Mul(number_or_old),
            _ => Operation::Add(number_or_old),
        };
        let test: Test = Test {
            divisible: block
                .get(3)
                .unwrap()
                .split("by ")
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            if_true: block
                .get(4)
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            if_false: block
                .get(5)
                .unwrap()
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        };

        Self {
            items: starting_items,
            operation: op,
            test,
            inspected: 0,
        }
    }

    fn throw(&self, index: usize, monkeys: &Vec<Monkey>) -> Vec<Monkey> {
        // return a new vector of monkeys where the throwing monkey lost the item and the receiving
        // monkey received it
        let inspected = self.inspected + 1;
        let item: Item = self.items.get(0).unwrap().clone();
        // println!("throwing item with worry {}", item.worry);
        // dbg!(&self.items);
        let new_worry_level = match &self.operation {
            Operation::Mul(value) => match value {
                Value::Old => item.worry * item.worry,
                Value::Num(a) => item.worry * a,
            },
            Operation::Add(value) => match value {
                Value::Old => item.worry + item.worry,
                Value::Num(a) => item.worry + a,
            },
        };
        let new_worry_level = ((new_worry_level / 3) as f32).round() as u64;
        let test_outcome: bool = match new_worry_level % self.test.divisible {
            0 => true,
            _ => false,
        };
        let index_of_receiving_monkey: usize;
        match test_outcome {
            true => index_of_receiving_monkey = self.test.if_true,
            false => index_of_receiving_monkey = self.test.if_false,
        }
        let throwing_monkey = Self {
            items: self.items[1..].to_owned(),
            inspected,
            ..*self
        };
        let receiving_monkey = monkeys.get(index_of_receiving_monkey).unwrap();
        let mut new_items = receiving_monkey.items.to_owned();
        new_items.push(Item{
            worry: new_worry_level,
        });
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


    fn throw_part_2(&self, index: usize, monkeys: &Vec<Monkey>) -> Vec<Monkey> {
        // return a new vector of monkeys where the throwing monkey lost the item and the receiving
        // monkey received it
        let inspected = self.inspected + 1;
        let item: Item = self.items.get(0).unwrap().clone();
        // println!("throwing item with worry {}", item.worry);
        // dbg!(&self.items);
        let new_worry_level = match &self.operation {
            Operation::Mul(value) => match value {
                Value::Old => item.worry * item.worry,
                Value::Num(a) => item.worry * a,
            },
            Operation::Add(value) => match value {
                Value::Old => item.worry + item.worry,
                Value::Num(a) => item.worry + a,
            },
        };
        let test_outcome: bool = match new_worry_level % self.test.divisible {
            0 => true,
            _ => false,
        };
        let index_of_receiving_monkey: usize;
        match test_outcome {
            true => index_of_receiving_monkey = self.test.if_true,
            false => index_of_receiving_monkey = self.test.if_false,
        }
        let throwing_monkey = Self {
            items: self.items[1..].to_owned(),
            inspected,
            ..*self
        };
        let receiving_monkey = monkeys.get(index_of_receiving_monkey).unwrap();
        let mut new_items = receiving_monkey.items.to_owned();
        let common_multiple: i64 = monkeys.iter().map(|m| m.test.divisible as i64).product();
        new_items.push(Item{
            worry: ((new_worry_level as i64) % common_multiple) as u64,
        });
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

#[derive(Clone, Debug)]
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
    fn next_part_2(&self, monkey_index: usize) -> State {
        let throwing_monkey = self.monkeys.get(monkey_index).unwrap();
        let new_monkeys = throwing_monkey.throw_part_2(monkey_index, &self.monkeys);
        State {
            monkeys: new_monkeys,
        }
    }

}

#[derive(Clone, Debug)]
struct Round {
    state: State,
    round: i32,
}

impl Round {
    fn next(&self) -> Round {
        let mut next_state: State = self.state.to_owned();
        for monkey_index in 0..self.state.monkeys.len() {
            let items_to_throw = &next_state.monkeys.get(monkey_index).unwrap().items;
            for _ in 0..items_to_throw.len() {
                next_state = next_state.next(monkey_index);
            }
        }
        Round {
            state: next_state,
            round: self.round + 1,
        }
    }

    fn next_part_2(&self) -> Round {
        let mut next_state: State = self.state.to_owned();
        for monkey_index in 0..self.state.monkeys.len() {
            let items_to_throw = &next_state.monkeys.get(monkey_index).unwrap().items;
            for _ in 0..items_to_throw.len() {
                next_state = next_state.next_part_2(monkey_index);
            }
        }
        Round {
            state: next_state,
            round: self.round + 1,
        }
    }
}

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let monkeys: Vec<Monkey> = input
        .split(|s| s == "")
        .map(|s| Monkey::from_block(&s.to_vec()))
        .collect();
    let starting_state = State { monkeys };
    let mut round = Round {
        state: starting_state,
        round: 0,
    };
    let number_of_rounds = 20;
    for i in 0..number_of_rounds {
        round = round.next();
    };
    let mut inspected_counts = round.state.monkeys.iter().map(|m| m.inspected).collect::<Vec<i32>>();
    inspected_counts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let total: i32 = inspected_counts[inspected_counts.len()-2..].iter().fold(1, |acc, x| acc * x);
    dbg!(total);
}


fn part_two() {
    let input = read_input("./input.txt", "\n");
    let monkeys: Vec<Monkey> = input
        .split(|s| s == "")
        .map(|s| Monkey::from_block(&s.to_vec()))
        .collect();
    let starting_state = State { monkeys };
    let mut round = Round {
        state: starting_state,
        round: 0,
    };
    let number_of_rounds = 10_000;
    for i in 0..number_of_rounds {
        round = round.next_part_2();
    };
    let mut inspected_counts = round.state.monkeys.iter().map(|m| m.inspected).collect::<Vec<i32>>();
    inspected_counts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let total: i64 = inspected_counts[inspected_counts.len()-2..].iter().fold(1, |acc, x| acc * (*x as i64));
    dbg!(total);
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
