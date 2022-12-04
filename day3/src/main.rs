extern crate core;

use crate::rucksack::Rucksack;
use advent_util::read_input;

mod rucksack;

fn main() {
    let input = read_input(3).unwrap();
    let rucksacks = input
        .lines()
        .map(|line| Rucksack::new(line))
        .collect::<Vec<_>>();
    let priority_sum: u32 = rucksacks.iter().map(|r| r.wrong_item_priority()).sum();

    let badge_priority_sum: u32 = rucksacks
        .as_slice()
        .chunks_exact(3)
        .map(|slice| {
            if let [a, b, c] = slice {
                a.badge_priority(&b, &c)
            } else {
                panic!("slice is not size 3")
            }
        })
        .sum();

    println!("Priority sum of all wrong types: {}", priority_sum);
    println!("Priority sum of badges: {}", badge_priority_sum);
}
