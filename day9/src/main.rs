use crate::rope::{Move, Pos, RopeMover};
use advent_util::read_input;
use std::collections::HashSet;

mod rope;

fn main() {
    let moves = read_input(9)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect::<Vec<_>>();

    let mut rope_mover = RopeMover::new(2);
    let different_positions = move_rope(&moves, &mut rope_mover);
    println!(
        "Amount of positions of tail for 2-rope: {}",
        different_positions.len()
    );

    let mut rope_mover = RopeMover::new(10);
    let different_positions = move_rope(&moves, &mut rope_mover);
    println!(
        "Amount of positions of tail for 10-rope: {}",
        different_positions.len()
    );
}

fn move_rope(moves: &[Move], rope_mover: &mut RopeMover) -> HashSet<Pos> {
    for mv in moves {
        rope_mover.move_head(*mv);
    }

    let mut different_positions = HashSet::new();
    for pos in rope_mover.tail_trail() {
        different_positions.insert(*pos);
    }
    different_positions
}
