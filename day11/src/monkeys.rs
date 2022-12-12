use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    throw: ThrowCondition,
}

impl Monkey {
    pub fn items_mut(&mut self) -> &mut VecDeque<u64> {
        &mut self.items
    }

    pub fn execute_operation(&self, item: u64, worry_reduction: WorryReduction) -> u64 {
        let new = match self.operation.arg {
            OpArg::Old => self.operation.op.execute(item, item),
            OpArg::Int(arg) => self.operation.op.execute(item, arg),
        };
        worry_reduction.reduce_worry(new)
    }

    pub fn next_monkey(&self, item: u64) -> usize {
        if item % self.throw.test == 0 {
            self.throw.monkey_success_id
        } else {
            self.throw.monkey_failure_id
        }
    }

    pub fn get_throw_test(&self) -> u64 {
        self.throw.test
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    op: Op,
    arg: OpArg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Mul,
    Plus,
}

impl Op {
    fn execute(&self, arg1: u64, arg2: u64) -> u64 {
        match self {
            Op::Mul => arg1 * arg2,
            Op::Plus => arg1 + arg2,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpArg {
    Old,
    Int(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThrowCondition {
    test: u64,
    monkey_success_id: usize,
    monkey_failure_id: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum WorryReduction {
    BoringMonkey,
    ModuleOperation(u64),
}

impl WorryReduction {
    fn reduce_worry(&self, worry: u64) -> u64 {
        match self {
            WorryReduction::BoringMonkey => worry / 3,
            WorryReduction::ModuleOperation(module) => worry % module,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse()
    }
}

struct Parser<'a> {
    input: &'a str,
    lines: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let lines = input.lines().map(|line| line.trim());
        let lines = Box::new(lines);
        Self { input, lines }
    }

    fn parse(mut self) -> Result<Monkey, <Monkey as FromStr>::Err> {
        let id = self.parse_id()?;
        let items = self.parse_items()?;
        let operation = self.parse_operation()?;
        let throw = self.parse_throw_condition()?;
        Ok(Monkey {
            id,
            items,
            operation,
            throw,
        })
    }

    fn parse_id(&mut self) -> Result<usize, <Monkey as FromStr>::Err> {
        let line = self.next_line(1)?;
        if let Some(rest) = line.strip_prefix("Monkey ") {
            rest.strip_suffix(":")
                .ok_or_else(|| format!("Malformed input!: 'Monkey {}'", rest))
                .and_then(|num| {
                    num.parse::<usize>()
                        .map_err(|err| format!("Cannot parse ID '{}': {}", num, err))
                })
        } else {
            Err(format!("Expected 'Monkey' but '{}' received", line))
        }
    }

    fn parse_items(&mut self) -> Result<VecDeque<u64>, <Monkey as FromStr>::Err> {
        self.next_line(2)
            .and_then(|line| {
                line.strip_prefix("Starting items: ")
                    .ok_or_else(|| format!("Expected 'Starting items', but '{}' recieved!", line))
            })
            .and_then(|items| {
                items
                    .split(", ")
                    .fold(Ok(VecDeque::new()), |res, item| match res {
                        Ok(mut vec) => item
                            .parse::<u64>()
                            .map_err(|err| {
                                format!("Cannot parse items '{}' because: {}", item, err)
                            })
                            .map(|item| {
                                vec.push_back(item);
                                vec
                            }),
                        Err(_) => res,
                    })
            })
    }

    fn parse_operation(&mut self) -> Result<Operation, <Monkey as FromStr>::Err> {
        self.next_line(3)
            .and_then(|line| {
                line.strip_prefix("Operation: new = old ").ok_or_else(|| {
                    format!("Expected 'Operation: new = old ' but '{}' received", line)
                })
            })
            .and_then(|op_and_num| {
                if let Some(rest) = op_and_num.strip_prefix("* ") {
                    Ok((Op::Mul, rest))
                } else if let Some(rest) = op_and_num.strip_prefix("+ ") {
                    Ok((Op::Plus, rest))
                } else {
                    Err(format!("Unknown operation: {}", op_and_num))
                }
            })
            .and_then(|(op, arg)| {
                if arg == "old" {
                    Ok(Operation {
                        op,
                        arg: OpArg::Old,
                    })
                } else {
                    arg.parse::<u64>()
                        .map_err(|err| {
                            format!("Cannot parse operation argument '{}' because: {}", arg, err)
                        })
                        .map(|value| Operation {
                            op,
                            arg: OpArg::Int(value),
                        })
                }
            })
    }

    fn parse_throw_condition(&mut self) -> Result<ThrowCondition, <Monkey as FromStr>::Err> {
        let line = self.next_line(4)?;
        let test = if let Some(rest) = line.strip_prefix("Test: divisible by ") {
            rest.parse::<u64>()
                .map_err(|err| format!("Cannot parse test '{}' because: {}", rest, err))
        } else {
            Err(format!(
                "Expected 'Test: divisible by ' but '{}' received",
                line
            ))
        }?;

        let line = self.next_line(5)?;
        let monkey_success_id = if let Some(rest) = line.strip_prefix("If true: throw to monkey ") {
            rest.parse::<usize>()
                .map_err(|err| format!("Cannot parse then branch '{}' because {}", rest, err))
        } else {
            Err(format!(
                "Expected 'If true: throw to monkey ' but '{}' received",
                line
            ))
        }?;

        let line = self.next_line(6)?;
        let monkey_failure_id = if let Some(rest) = line.strip_prefix("If false: throw to monkey ")
        {
            rest.parse::<usize>()
                .map_err(|err| format!("Cannot parse else branch '{}' because {}", rest, err))
        } else {
            Err(format!(
                "Expected 'If false: throw to monkey ' but {} received",
                line
            ))
        }?;
        Ok(ThrowCondition {
            test,
            monkey_success_id,
            monkey_failure_id,
        })
    }

    fn next_line(&mut self, expected_amount: usize) -> Result<&str, String> {
        self.lines.next().ok_or_else(|| {
            format!(
                "Malformed input! Expected at least {} lines: {}",
                expected_amount, self.input
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::monkeys::{Monkey, Op, OpArg, Operation, Parser, ThrowCondition};
    use std::collections::VecDeque;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

    #[test]
    fn parse_id() {
        let mut parser = Parser::new(INPUT);
        assert_eq!(parser.parse_id(), Ok(0));
    }

    #[test]
    fn parse_items() {
        let mut parser = Parser::new(INPUT);
        parser.lines = Box::new(parser.lines.skip(1));

        assert_eq!(parser.parse_items(), Ok(VecDeque::from([79, 98])));
    }

    #[test]
    fn parse_operation() {
        let mut parser = Parser::new(INPUT);
        parser.lines = Box::new(parser.lines.skip(2));
        assert_eq!(
            parser.parse_operation(),
            Ok(Operation {
                op: Op::Mul,
                arg: OpArg::Int(19)
            })
        )
    }

    #[test]
    fn parse_throw_condition() {
        let mut parser = Parser::new(INPUT);
        parser.lines = Box::new(parser.lines.skip(3));
        assert_eq!(
            parser.parse_throw_condition(),
            Ok(ThrowCondition {
                test: 23,
                monkey_success_id: 2,
                monkey_failure_id: 3,
            })
        )
    }

    #[test]
    fn parse() {
        let parser = Parser::new(INPUT);
        assert_eq!(
            parser.parse(),
            Ok(Monkey {
                id: 0,
                items: VecDeque::from([79, 98]),
                operation: Operation {
                    op: Op::Mul,
                    arg: OpArg::Int(19)
                },
                throw: ThrowCondition {
                    test: 23,
                    monkey_success_id: 2,
                    monkey_failure_id: 3
                }
            })
        )
    }

    #[test]
    fn parse_old_dependent_arg() {
        let mut parser = Parser::new("  Operation: new = old + old");
        assert_eq!(
            parser.parse_operation(),
            Ok(Operation {
                op: Op::Plus,
                arg: OpArg::Old
            })
        )
    }
}
