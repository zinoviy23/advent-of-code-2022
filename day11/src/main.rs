use crate::keep_away::KeepAway;
use crate::monkeys::{Monkey, WorryReduction};
use advent_util::read_input;

mod keep_away;
mod monkeys;

const NEW_GAME: usize = 10000;

fn main() {
    let mut monkeys = read_input(11)
        .unwrap()
        .split("\n\n")
        .map(|input| input.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();
    let mut other_monkeys = monkeys.clone();
    // dbg!(&monkeys);

    let monkey_business = {
        let mut keep_away = KeepAway::new(&mut monkeys, WorryReduction::BoringMonkey);
        keep_away.play(20)
    };
    println!("Monkey business: {}", monkey_business);

    let monkey_business = {
        let module = WorryReduction::ModuleOperation(
            other_monkeys.iter().map(|m| m.get_throw_test()).product(),
        );
        let mut keep_away = KeepAway::new(&mut other_monkeys, module);
        keep_away.play(NEW_GAME)
    };
    println!(
        "Monkey business after {} rounds: {}",
        NEW_GAME, monkey_business
    );
}
