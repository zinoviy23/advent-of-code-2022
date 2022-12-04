extern crate core;

use crate::rucksack::Rucksack;
use advent_util::read_input;

mod rucksack;

fn main() {
    let input = read_input(3).unwrap();
    let priority_sum: u32 = input
        .lines()
        .map(|line| Rucksack::new(line))
        .map(|r| r.wrong_item_priority())
        .sum();
    println!("Priority sum of all wrong types: {}", priority_sum);
}
