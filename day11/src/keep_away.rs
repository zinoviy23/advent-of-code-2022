use crate::monkeys::{Monkey, WorryReduction};

#[derive(Debug)]
pub struct KeepAway<'a> {
    monkeys: &'a mut [Monkey],
    worry_reduction: WorryReduction,
}

#[derive(Debug, Default, Clone)]
struct Statistics {
    items_inspected: usize,
}

impl<'a> KeepAway<'a> {
    pub fn new(monkeys: &'a mut [Monkey], worry_reduction: WorryReduction) -> Self {
        Self {
            monkeys,
            worry_reduction,
        }
    }

    pub fn play(&mut self, rounds: usize) -> usize {
        let mut statistics = vec![Statistics::default(); self.monkeys.len()];
        for _ in 0..rounds {
            for monkey_id in 0..self.monkeys.len() {
                while let Some(item) = self.monkeys[monkey_id].items_mut().pop_front() {
                    statistics[monkey_id].items_inspected += 1;

                    let (new, new_monkey) = {
                        let monkey = &self.monkeys[monkey_id];
                        let new = monkey.execute_operation(item, self.worry_reduction);
                        let new_monkey = monkey.next_monkey(new);
                        (new, new_monkey)
                    };
                    self.monkeys[new_monkey].items_mut().push_back(new);
                }
            }
        }

        statistics.sort_by(|item, other| other.items_inspected.cmp(&item.items_inspected));
        assert!(statistics.len() >= 2);
        // dbg!(&statistics);
        statistics[0].items_inspected * statistics[1].items_inspected
    }
}
