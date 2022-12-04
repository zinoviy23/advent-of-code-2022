use crate::assigment::Assignments;
use advent_util::read_input;

mod assigment;

fn main() {
    let input = read_input(4).unwrap();
    let assignments: Vec<Assignments> = input
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Cannot parse assigment {}", line))
        })
        .collect();

    let count_of_overlaps = assignments
        .iter()
        .filter(|assigment| assigment.does_one_contain_another())
        .count();

    let count_of_intersects = assignments
        .iter()
        .filter(|assigment| assigment.intersects())
        .count();

    println!("Count of full overlaps: {}", count_of_overlaps);
    println!("Count of intersects: {}", count_of_intersects);
}
