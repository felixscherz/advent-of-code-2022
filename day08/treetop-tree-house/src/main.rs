mod grid;
mod utils;
use crate::grid::*;
use crate::utils::read_input;
use std::collections::HashMap;
use std::collections::HashSet;

fn part_one() {
    let input = read_input("./input.txt", "\n");
    let grid = Grid::from_input(&input);
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..grid.height {
        assert!(grid.tree_visible(i, 0, Direction::West));
        assert!(grid.tree_visible(i, grid.width - 1, Direction::East));
    }
    for j in 0..grid.width {
        assert!(grid.tree_visible(0, j, Direction::North));
        assert!(grid.tree_visible(grid.height - 1, j, Direction::South));
    }
    for i in 0..grid.height {
        for j in 0..grid.width {
            for dir in vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            {
                if grid.tree_visible(i, j, dir) {
                    visible_trees.insert((i, j));
                }
            }
        }
    }
    dbg!(&visible_trees.len());
}

fn part_two() {
    let input = read_input("./input.txt", "\n");
    let grid = Grid::from_input(&input);
    let mut treehouse_locations: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..grid.height {
        for j in 0..grid.width {
            let mut scores: Vec<usize> = Vec::new();
            for dir in vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            {
                scores.push(grid.trees_visible(i, j, dir));
            }
            let total: usize = scores.iter().product();
            treehouse_locations.insert((i, j), total);
            dbg!((i, j), &scores, total);
        }
    }
    let mut max_pos: (usize, usize) = (0, 0);
    let mut max_score: usize = 0;
    dbg!(&treehouse_locations);
    for (pos, score) in treehouse_locations.into_iter() {
        if pos == (3, 2) {
            dbg!(&pos);
            dbg!(&score);
        }
        if score > max_score {
            max_score = score;
            max_pos = pos;
        }
    }
    dbg!(grid.trees_visible(3, 2, Direction::North));
    dbg!(grid.trees_visible(3, 2, Direction::East));
    dbg!(grid.trees_visible(3, 2, Direction::West));
    dbg!(grid.trees_visible(3, 2, Direction::South));
    dbg!(max_pos);
    dbg!(max_score);
}

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();
}
