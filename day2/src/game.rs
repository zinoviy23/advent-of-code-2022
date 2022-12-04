use std::cmp::Ordering;
use std::str::FromStr;
use enum_iterator::{next, previous, Sequence};

#[derive(Eq, PartialEq, Ord, Copy, Clone, Sequence)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn wins(&self, other: &Self) -> bool {
        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => true,
            _ => false,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn opponent_to_win(&self) -> Hand {
        previous(self).unwrap_or(Hand::Scissors)
    }

    fn opponent_to_lose(&self) -> Hand {
        next(self).unwrap_or(Hand::Rock)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.wins(other) {
            Ordering::Greater
        } else if other.wins(self) {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }
}

struct Opponent(Hand);

impl AsRef<Hand> for Opponent {
    fn as_ref(&self) -> &Hand {
        &self.0
    }
}

struct Player(Hand);

impl AsRef<Hand> for Player {
    fn as_ref(&self) -> &Hand {
        &self.0
    }
}

trait GameAttender : AsRef<Hand> {
    fn wins(&self, other_hand: impl AsRef<Hand>) -> bool;

    fn play(&self, other_hand: impl AsRef<Hand>) -> GameResult {
        let other_hand = other_hand.as_ref();
        if self.as_ref().wins(other_hand) {
            GameResult::Win
        } else if other_hand.wins(self.as_ref()) {
            GameResult::Loss
        } else {
            GameResult::Draw
        }
    }
}

impl <T> GameAttender for T where T : AsRef<Hand> {
    fn wins(&self, other_hand: impl AsRef<Hand>) -> bool {
        self.as_ref().wins(other_hand.as_ref())
    }
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn points(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}

pub struct Game {
    opponent: Opponent,
    player: Player,
    expected_result: GameResult,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError;

impl FromStr for Opponent {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Opponent(Hand::Rock)),
            "B" => Ok(Opponent(Hand::Paper)),
            "C" => Ok(Opponent(Hand::Scissors)),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for Player {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Player(Hand::Rock)),
            "Y" => Ok(Player(Hand::Paper)),
            "Z" => Ok(Player(Hand::Scissors)),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for GameResult {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseError)
        }
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let opponent: Opponent = split.next().ok_or(ParseError).and_then(|s| s.parse())?;
        let second_column = split.next().ok_or(ParseError);
        let player: Player = second_column.and_then(|s| s.parse())?;
        let expected_result: GameResult = second_column.and_then(|s| s.parse())?;
        Ok(Game { opponent, player, expected_result })
    }
}

impl Game {
    pub fn player_score(&self) -> u32 {
        let game_result = self.player.play(&self.opponent).points();
        self.player.as_ref().points() + game_result
    }

    pub fn score_with_guessing(&self) -> u32 {
        self.expected_result.points() + self.guess_player_hand().points()
    }

    fn guess_player_hand(&self) -> Hand {
        match self.expected_result {
            GameResult::Win => self.opponent.as_ref().opponent_to_lose(),
            GameResult::Draw => *self.opponent.as_ref(),
            GameResult::Loss => self.opponent.as_ref().opponent_to_win(),
        }
    }
}