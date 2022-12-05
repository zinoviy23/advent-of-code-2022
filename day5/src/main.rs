use crate::crates::{Crates, Move};
use advent_util::read_input;

mod crates;

fn main() {
    let input = read_input(5).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let crates_lines = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| *line)
        .collect::<Vec<_>>();

    let mut crates = Crates::from_lines(crates_lines.as_slice());
    let mut crates_new = crates.clone();

    let moves_lines = &lines[crates_lines.len() + 1..];
    let moves: Vec<Move> = moves_lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    for current_move in moves.iter() {
        crates.move_crates(&current_move);
        crates_new.move_crates_new(&current_move);
    }

    let peeks: String = peeks_as_string(&crates);
    let peeks_new: String = peeks_as_string(&crates_new);
    println!("Result peeks after moves: {}", peeks);
    println!("Result peeks after new moves: {}", peeks_new);
}

fn peeks_as_string(crates: &Crates) -> String {
    crates
        .peeks()
        .iter()
        .map(|peek| {
            peek.map(|peek| format!("{}", peek))
                .unwrap_or("".to_string())
        })
        .collect()
}
