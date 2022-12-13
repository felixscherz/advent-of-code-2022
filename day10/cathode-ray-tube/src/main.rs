use std::fs;
fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

#[derive(Debug)]
struct AddX {
    value: i32,
}

#[derive(Debug)]
enum Command {
    AddX(AddX),
    NoOp,
}

impl Command {
    fn from_line(s: &String) -> Self {
        match s.as_str() {
            "noop" => Command::NoOp,
            _ => Command::AddX(AddX {
                value: s.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap(),
            }),
        }
    }
}

fn part_one() {
    let input: Vec<String> = read_input("./input.txt", "\n");
    let commands: Vec<Command> = input.iter().map(|l| Command::from_line(l)).collect();
    let mut i: i32 = 0;
    let mut x: i32 = 1;
    let mut is: Vec<i32> = Vec::new();
    let mut xs: Vec<i32> = Vec::new();
    for cmd in commands.iter() {
        i += 1;
        is.push(i);
        xs.push(x);
        match cmd {
            Command::NoOp => (),
            Command::AddX(addx) => {
                i += 1;
                x += addx.value;
                is.push(i);
                xs.push(x);
            }
        }
    }
    let cycles: Vec<i32> = (20..=220).step_by(40).collect();
    let signal_strengths: Vec<i32> = is
        .iter()
        .zip(xs.iter())
        .filter(|(i, _)| cycles.contains(i))
        .map(|(i, x)| i * x)
        .collect();
    dbg!(&signal_strengths);
    let signal_strength: i32 = is
        .iter()
        .zip(xs.iter())
        .filter(|(i, _)| cycles.contains(i))
        .map(|(i, x)| i * x)
        .sum();

    dbg!(signal_strength);
}

fn part_two() {
    let input: Vec<String> = read_input("./input.txt", "\n");
    let commands: Vec<Command> = input.iter().map(|l| Command::from_line(l)).collect();
    let mut i: i32 = 0;
    let mut x: i32 = 1;
    let mut is: Vec<i32> = Vec::new();
    let mut xs: Vec<i32> = Vec::new();
    for cmd in commands.iter() {
        i += 1;
        is.push(i);
        xs.push(x);
        match cmd {
            Command::NoOp => (),
            Command::AddX(addx) => {
                i += 1;
                x += addx.value;
                is.push(i);
                xs.push(x);
            }
        }
    }
    let cycles: Vec<i32> = (20..=220).step_by(40).collect();
    let width: i32 = 40;
    let pixels: Vec<String> = is
        .iter()
        .zip(xs.iter())
        .map(|(i, x)| {
            let pixel = i % width;
            if pixel == (x-1) || pixel == *x || pixel == (x+1) {
                return "#".to_string();
            }
            " ".to_string()
        })
        .collect();

    let rows: Vec<String> = pixels.chunks(width.try_into().unwrap()).map(|ps| ps.join("")).collect();
    dbg!(rows);

}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
