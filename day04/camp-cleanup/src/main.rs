use std::fs;

struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Section) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

struct Group {
    sections: Vec<Section>,
}

impl Group {
    fn from_line(line: &String) -> Self {
        Group {
            sections: line
                .split(",")
                .map(|s| {
                    let boundaries: Vec<i32> =
                        s.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
                    Section {
                        start: *boundaries.get(0).unwrap(),
                        end: *boundaries.get(1).unwrap(),
                    }
                })
                .collect(),
        }
    }

    fn has_contained_section(&self) -> bool {
        for (i, sec) in self.sections.iter().enumerate() {
            for (_, other) in self.sections.iter().enumerate().filter(|&(j, _)| j != i) {
                if sec.contains(&other) {
                    return true;
                }
            }
        }
        false
    }

    fn has_overlapping_section(&self) -> bool {
        for (i, sec) in self.sections.iter().enumerate() {
            for (_, other) in self.sections.iter().enumerate().filter(|&(j, _)| j != i) {
                if sec.overlaps(&other) {
                    return true;
                }
            }
        }
        false

    }
}

fn read_input<'a>(filepath: &'a str, sep: &'a str) -> Vec<String> {
    // yay lifetime parameters
    let contents: String = fs::read_to_string(filepath).expect("Should be able to read {filepath}");
    let mut lines: Vec<&str> = contents.split(sep).collect();
    lines.pop();
    lines.iter().map(|s| s.to_string()).collect()
}

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let groups = input
        .iter()
        .map(|l| Group::from_line(&l))
        .collect::<Vec<Group>>();
    let number_of_pairs = groups.iter().map(|g| g.has_contained_section()).filter(|x| *x).count();
    println!("Number of contained pairs {number_of_pairs}");
}

fn part_two() {
    let input = read_input("./input.txt", "\n");
    let groups = input
        .iter()
        .map(|l| Group::from_line(&l))
        .collect::<Vec<Group>>();
    let number_of_overlapping_pairs = groups.iter().map(|g| g.has_overlapping_section()).filter(|x| *x).count();
    println!("Number of overlapping pairs {number_of_overlapping_pairs}");
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
