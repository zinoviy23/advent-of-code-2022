use crate::game::Game;
use advent_util::read_input;

mod game;

fn main() {
    let input = read_input(2).unwrap();
    let games = input
        .lines()
        .map(|line| {
            let game: Game = line.parse().expect(&format!("Cannot parse line: {}", line));
            game
        })
        .collect::<Vec<_>>();

    let player_score: u32 = games.iter().map(|game| game.player_score()).sum();
    println!("Player score for strategy: {}", player_score);
}
