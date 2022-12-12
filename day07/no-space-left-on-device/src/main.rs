use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Cd {
    arg: String,
}

impl Cd {
    fn from_line(line: &String) -> Self {
        Self {
            arg: line.split(" ").nth(2).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
struct Ls<'a> {
    output: Vec<&'a Object>,
}

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd),
    Ls(Ls<'a>),
}

#[derive(Debug)]
enum Object {
    File { filename: String, size: i32 },
    Dir { dirname: String },
}

impl Object {
    fn file_from_line(line: &String) -> Self {
        Self::File {
            filename: line.split(" ").nth(1).unwrap().to_string(),
            size: line.split(" ").nth(0).unwrap().parse::<i32>().unwrap(),
        }
    }

    fn dir_from_line(line: &String) -> Self {
        Self::Dir {
            dirname: line.split(" ").nth(0).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
enum Line<'a> {
    Command(Command<'a>),
    Object(Object),
}

fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn parse_input_lines(lines: &Vec<String>) -> Vec<Line> {
    lines
        .iter()
        .map(|l| match l.chars().next().unwrap() {
            '$' => match l.chars().nth(2).unwrap() {
                'c' => Line::Command(Command::Cd(Cd::from_line(&l))),
                _ => Line::Command(Command::Ls(Ls { output: Vec::new() })),
            },
            _ => match l.chars().next().unwrap() {
                'd' => Line::Object(Object::dir_from_line(&l)),
                _ => Line::Object(Object::file_from_line(&l)),
            },
        })
        .collect()
}

fn track_current_dir(lines: &Vec<Line>) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut current_dir: String = "".to_string();
    for line in lines.iter() {
        match line {
            Line::Command(Command::Cd(cd)) => match cd.arg.as_str() {
                ".." => {
                    stack.pop();
                    current_dir = stack[stack.len() - 1].clone();
                }
                s => {
                    current_dir = s.to_string();
                    stack.push(current_dir.clone())
                }
            },
            _ => (),
        }
        dirs.push(
            stack
                .iter()
                .map(|s| (("-".to_owned() + s).to_owned() + "-").to_owned())
                .collect::<String>(),
        );
    }
    dirs
}

fn calc_dir_size(dirname: &String, dir_size: &HashMap<String, i32>) -> i32 {
    let mut total_dir_size: i32 = 0;
    for (key, size) in dir_size.iter() {
        if key.starts_with(dirname) {
            total_dir_size += size;
        }
    }
    total_dir_size
}

fn add_up_files(lines: &Vec<Line>, dirs: &Vec<String>) -> HashMap<String, i32> {
    let mut dir_size: HashMap<String, i32> = HashMap::new();
    for (line, dir) in lines.iter().zip(dirs) {
        match line {
            Line::Object(Object::File { filename, size }) => {
                *dir_size.entry(dir.clone()).or_insert(0) += size;
            }
            Line::Object(Object::Dir { dirname }) => {
                *dir_size.entry(dir.clone()).or_insert(0) += 0;
            }
            _ => (),
        }
    }
    dir_size
}

fn add_up_subdirs(dir_sizes: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut total_dir_sizes: HashMap<String, i32> = HashMap::new();
    for (dirname, _) in dir_sizes {
        total_dir_sizes.insert(dirname.clone(), calc_dir_size(&dirname, &dir_sizes));
    }
    total_dir_sizes
}

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let lines = parse_input_lines(&input);
    let dirs = track_current_dir(&lines);
    dbg!(&dirs);
    let dir_sizes = add_up_files(&lines, &dirs);
    let total_dir_sizes = add_up_subdirs(&dir_sizes);
    let sum = total_dir_sizes
        .iter()
        .filter(|(_, size)| **size <= 100_000)
        .map(|(_, size)| size)
        .sum::<i32>();
    dbg!(sum);
}

fn part_two() {
    let input = read_input("./input.txt", "\n");
    let lines = parse_input_lines(&input);
    let dirs = track_current_dir(&lines);
    dbg!(&dirs);
    let dir_sizes = add_up_files(&lines, &dirs);
    let total_dir_sizes = add_up_subdirs(&dir_sizes);
    let total_size = total_dir_sizes.get("-/-").unwrap();
    let unused_space = 70000000 - total_size;
    let size_to_free = 30000000 - unused_space;
    let mut dir_to_free = "-/-".to_string();
    for (dir, size) in total_dir_sizes.iter() {
        if *size >= size_to_free {
            if *size < *total_dir_sizes.get(&dir_to_free).unwrap() {
                dir_to_free = dir.clone();
            }
        }
    }
    dbg!(&dir_to_free);
    dbg!(total_dir_sizes.get(&dir_to_free).unwrap());
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
