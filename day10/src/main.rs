use crate::instruction::{Cpu, Instruction};
use advent_util::read_input;

mod instruction;

fn main() {
    let program = read_input(10)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let cpu = Cpu;
    let sum_of_signal_strengths = cpu.execute(&program);

    println!("Sum of signal strengths: {}", sum_of_signal_strengths);
}
