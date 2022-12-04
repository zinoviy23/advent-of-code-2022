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
    let player_strategy_score: u32 = games.iter().map(|game| game.score_with_guessing()).sum();
    println!("Player score for strategy: {}", player_score);
    println!("Player score with strategy: {}", player_strategy_score);
}
