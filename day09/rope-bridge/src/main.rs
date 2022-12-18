mod rope;
mod utils;

use crate::rope::*;
use crate::utils::read_input;
use std::collections::HashSet;
fn part_one() {
    let input = read_input("./input.txt", "\n");
    let moves: Vec<Move> = input.iter().map(|l| Move::from_line(l)).collect();
    let moves: Vec<Move>= moves.into_iter().flat_map(|m| m.break_down()).collect();
    let starting_point = Point { x: 0, y: 0 };
    let mut rope: Rope = Rope {
        head: starting_point,
        tail: starting_point,
    };
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    tail_positions.insert(rope.tail.to_tuple());

    for mv in moves.iter() {
        rope = rope.move_rope(mv);
        tail_positions.insert(rope.tail.to_tuple());
    }
    dbg!(&tail_positions.len());

}

fn main() {
    println!("Hello, world!");
    part_one();
}
