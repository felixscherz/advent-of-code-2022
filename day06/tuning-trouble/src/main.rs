use std::fs;
use std::collections::HashSet;

fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn first_n_distinct_after(text: &String, n: &usize) -> usize {
    let mut index: usize = 0;
    for i in (*n-1)..(text.chars().count()) {
        let mut chars: HashSet<char> = HashSet::new();
        for j in i-(n-1)..i+1 {
            chars.insert(text.chars().nth(j).unwrap());
        }
        if chars.len() == *n {
            index = i;
            break;
        }
    }
    index+1
}


fn part_one() {
    let input = read_input("./input.txt", "\n")[0].clone();
    let n = 4;
    let index = first_n_distinct_after(&input, &n);
    println!("for part one: {:?}", index);
}

fn part_two() {
    let input = read_input("./input.txt", "\n")[0].clone();
    let n = 14;
    let index = first_n_distinct_after(&input, &n);
    println!("for part two: {:?}", index);
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
