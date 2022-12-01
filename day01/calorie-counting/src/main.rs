use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't read file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let lines_per_elf: Vec<Vec<&str>> = lines.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == "" || acc.is_empty() {
            // for first iteration and at every split add new empty vec
            acc.push(Vec::new());
        }
        if x != "" {
            acc.last_mut().unwrap().push(x);
        }
        acc
    });
    let calories_list_per_elf: Vec<Vec<i32>> = lines_per_elf
        .into_iter()
        .map(|s| s.into_iter().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let calories_list_per_elf = calories_list_per_elf
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<i32>>>();

    let mut calories_sum_per_elf: Vec<i32> = calories_list_per_elf
        .into_iter()
        .map(|x| x.into_iter().fold(0, |acc, y| acc + y))
        .collect();

    let mut calories_of_top_elves: Vec<i32> = Vec::new();
    let number_of_top_elves = 3;
    for _ in 0..number_of_top_elves {
        let (max_index, max_value) = find_max(&calories_sum_per_elf);
        calories_of_top_elves.push(max_value);
        calories_sum_per_elf.remove(max_index);
    }
    println!("Elves with most calories are carrying {:?} calories", calories_of_top_elves);
    let sum_of_calories_of_top_elves: i32 = calories_of_top_elves.iter().sum();
    println!("Sum of calories of top {number_of_top_elves} is {sum_of_calories_of_top_elves} calories");
    
}

fn find_max(v: &Vec<i32>) -> (usize, i32) {
    let mut max_value = 0;
    let mut max_index = 0;
    for (index, &x) in v.iter().enumerate() {
        if x > max_value {
            max_value = x;
            max_index = index;
        }
    }
    (max_index, max_value)

}
