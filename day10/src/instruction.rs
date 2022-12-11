use crate::instruction::Instruction::{Addx, Noop};
use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> impl Iterator<Item = ()> {
        match self {
            Noop => repeat(()).take(1),
            Addx(_) => repeat(()).take(2),
        }
    }

    fn apply_to_register(&self, register: &mut i32) {
        if let Addx(value) = self {
            *register += value;
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(_) = s.strip_prefix("noop") {
            Ok(Noop)
        } else if let Some(rest) = s.strip_prefix("addx ") {
            rest.parse::<i32>()
                .map_err(|err| format!("Cannot parse addx argument '{}': {}", rest, err))
                .map(|v| Addx(v))
        } else {
            Err(format!("Unknown command: {}", s))
        }
    }
}

pub struct Cpu;

impl Cpu {
    pub fn execute(&self, program: &[Instruction]) -> i32 {
        let mut cycle = 1i32;
        let mut register = 1i32;

        let signal_strength_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
        let mut signal_strength = 0;
        for instruction in program {
            for _ in instruction.cycles() {
                if signal_strength_cycles.contains(&cycle) {
                    signal_strength += cycle * register;
                }
                cycle += 1;
            }
            instruction.apply_to_register(&mut register);
        }

        signal_strength
    }
}
