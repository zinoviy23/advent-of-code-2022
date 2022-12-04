use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord)]
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

trait GameAttender {
    fn wins(&self, other_hand: impl AsRef<Hand>) -> bool;
}

impl <T> GameAttender for T where T : AsRef<Hand> {
    fn wins(&self, other_hand: impl AsRef<Hand>) -> bool {
        self.as_ref().wins(other_hand.as_ref())
    }
}

pub struct Game {
    opponent: Opponent,
    player: Player,
}

#[derive(Debug)]
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

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let opponent: Opponent = split.next().ok_or(ParseError).and_then(|s| s.parse())?;
        let player: Player = split.next().ok_or(ParseError).and_then(|s| s.parse())?;
        Ok(Game { opponent, player })
    }
}

impl Game {
    pub fn player_score(&self) -> u32 {
        let game_result: u32 = if self.player.wins(&self.opponent) {
            6
        } else if self.opponent.wins(&self.player) {
            0
        } else {
            3
        };
        self.player.as_ref().points() + game_result
    }
}
