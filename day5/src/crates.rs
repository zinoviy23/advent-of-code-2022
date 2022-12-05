use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
struct Crate(char);

type CratesStack = Vec<Crate>;

#[derive(Debug, Clone)]
pub struct Crates {
    stacks: Vec<CratesStack>,
}

impl Crates {
    pub fn from_lines(lines: &[&str]) -> Self {
        let titles = lines.last().expect("No title line!");
        let last_title: usize = titles
            .split_whitespace()
            .last()
            .expect("At least one title should be")
            .parse()
            .unwrap();

        let mut stacks = vec![CratesStack::new(); last_title];

        for crates_level in lines.iter().rev().skip(1) {
            let crates_level: CratesLevel = crates_level.parse().unwrap();
            if crates_level.0.len() > stacks.len() {
                panic!("Wrong level!")
            }
            for (i, current_crate) in crates_level.0.iter().enumerate() {
                if let Some(current_crate) = current_crate {
                    stacks[i].push(current_crate.clone());
                }
            }
        }

        Self { stacks }
    }

    pub fn move_crates(&mut self, move_info: &Move) {
        for _ in 0..move_info.crate_count {
            let crate_to_move = self.stacks[move_info.stack_from]
                .pop()
                .expect(&format!("Cannot execute move: {:?}", move_info));
            self.stacks[move_info.stack_to].push(crate_to_move);
        }
    }

    pub fn move_crates_new(&mut self, move_info: &Move) {
        let from = &mut self.stacks[move_info.stack_from];
        let mut crates_to_move = from.split_off(from.len() - move_info.crate_count);
        self.stacks[move_info.stack_to].append(&mut crates_to_move);
    }

    pub fn peeks(&self) -> Vec<Option<char>> {
        self.stacks
            .iter()
            .map(|stack| stack.last())
            .map(|peek| peek.map(|peek_crate| peek_crate.0))
            .collect()
    }
}

#[derive(PartialEq, Debug)]
struct CratesLevel(Vec<Option<Crate>>);

#[derive(PartialEq, Debug)]
struct OptionCrate(Option<Crate>);

impl FromStr for OptionCrate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.strip_prefix('[')
                .and_then(|s| s.strip_suffix("] ").or_else(|| s.strip_suffix(']')))
                .and_then(|s| s.chars().next())
                .map(|c| Crate(c)),
        ))
    }
}

impl FromStr for CratesLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.as_bytes()
                .chunks(4)
                .map(|bytes| std::str::from_utf8(bytes).unwrap())
                .map(|s| s.parse::<OptionCrate>().unwrap().0)
                .collect(),
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    crate_count: usize,
    stack_from: usize,
    stack_to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let crate_count: usize = split.nth(1).unwrap().parse().unwrap();
        let stack_from: usize = split.nth(1).unwrap().parse().unwrap();
        let stack_from = stack_from - 1;
        let stack_to: usize = split.nth(1).unwrap().parse().unwrap();
        let stack_to = stack_to - 1;

        Ok(Self {
            crate_count,
            stack_from,
            stack_to,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::{Crate, CratesLevel, Move, OptionCrate};

    #[test]
    fn crates_level_parse() {
        let str = "            [R] [N]     [T] [T] [C]";
        assert_eq!(
            str.parse(),
            Ok(CratesLevel(vec![
                None,
                None,
                None,
                Some(Crate('R')),
                Some(Crate('N')),
                None,
                Some(Crate('T')),
                Some(Crate('T')),
                Some(Crate('C'))
            ]))
        );
    }

    #[test]
    fn crate_parse() {
        assert_eq!("[x] ".parse(), Ok(OptionCrate(Some(Crate('x')))));
    }

    #[test]
    fn parse_move() {
        assert_eq!(
            "move 1 from 2 to 3".parse(),
            Ok(Move {
                crate_count: 1,
                stack_from: 1,
                stack_to: 2
            })
        );
    }
}
