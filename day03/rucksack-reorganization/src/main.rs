use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug)]
struct Rucksack {
    left: String,
    right: String,
}

impl Rucksack {
    fn from_line(line: String) -> Self {
        let length = line.len();
        Rucksack {
            left: line.get(0..(length / 2)).unwrap().to_string(),
            right: line.get((length / 2)..).unwrap().to_string(),
        }
    }

    fn find_common_item(&self) -> Option<char> {
        let mut char_count = HashMap::new();
        for c in self.left.chars() {
            char_count.insert(c, 1);
        }
        for c in self.right.chars() {
            if char_count.contains_key(&c) {
                return Some(c);
            }
        }
        None
    }
}


fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn get_priority(c: &char) -> i32 {
    let alphabet: Vec<char> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .collect();
    let scores: Vec<i32> = Vec::from_iter(1..(alphabet.len() + 1))
        .iter()
        .map(|x| *x as i32)
        .collect();
    let score_lookup: HashMap<char, i32> = alphabet.into_iter().zip(scores.into_iter()).collect();
    *score_lookup.get(&c).unwrap()
}

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let rucksacks: Vec<Rucksack> = input
        .iter()
        .map(|l| Rucksack::from_line(l.to_string()))
        .collect();
    let common_items: Vec<char> = rucksacks
        .iter()
        .map(|r| r.find_common_item().unwrap())
        .collect();

    let sum_of_priorities: i32 = common_items.iter().map(|x| get_priority(&x)).sum();

    println!("Sum of priorities: {}", sum_of_priorities);
}

#[derive(Clone, Debug)]
struct ElfGroup {
    rucksacks: Vec<Rucksack>,
}

impl ElfGroup {
    fn find_common_item(&self) -> Option<char> {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        for rucksack in self.rucksacks.iter() {
            let items: String = rucksack.left.to_owned() + &rucksack.right;
            let items: HashSet<char> = items.chars().collect();
            for c in items {
                *char_count.entry(c).or_insert(0) += 1;
            }
        }
        for (c, count) in char_count.iter() {
            if *count == 3 {
                return Some(*c);
            }
        }
        None
    }
}

fn part_two() {
    let input = read_input("./input.txt", "\n");
    let rucksacks: Vec<Rucksack> = input
        .iter()
        .map(|l| Rucksack::from_line(l.to_string()))
        .collect();
    let elf_groups: Vec<ElfGroup> = (0..rucksacks.len()).step_by(3).map(|i| {
        ElfGroup {
            rucksacks: rucksacks[i..i+3].to_vec()
        }
    }).collect();
    let group_items: Vec<char> = elf_groups.iter().map(|g| g.find_common_item().unwrap()).collect();
    let sum_of_priorities: i32 = group_items.iter().map(|x| get_priority(&x)).sum();
    println!("Sum of group item priorities {}", sum_of_priorities);
}


fn main() {
    part_one();
    part_two();
}
