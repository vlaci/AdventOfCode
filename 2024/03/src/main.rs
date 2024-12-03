// SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
//
// SPDX-License-Identifier: EUPL-1.2

use color_eyre::eyre::{Report, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let parsed: Memory = INPUT.parse()?;
    let part1 = parsed.exec();
    println!("The answer to the 1st part is {part1}");

    let part2 = parsed.exec_with_toggle();
    println!("The answer to the 2nd part is {part2}");
    Ok(())
}

static INPUT: &str = include_str!("../input");

#[derive(PartialEq, Debug)]
enum Instruction {
    Mul(usize, usize),
    Dont,
    Do,
}

#[derive(PartialEq, Debug)]
struct Memory(Vec<Instruction>);

impl std::str::FromStr for Memory {
    type Err = Report;
    fn from_str(input: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule memory() -> Memory
                    = i:instruction()+ garbage()* { Memory(i.into_iter().flatten().collect()) }
                rule instruction() -> Option<Instruction>
                    = (m:(mul() / dont() / do()) { Some(m) }) / (garbage() { None })
                rule mul() -> Instruction
                    = "mul(" a:number() "," b:number() ")" { Instruction::Mul(a, b) }
                rule dont() -> Instruction
                    = "don't()" { Instruction::Dont }
                rule do() -> Instruction
                    = "do()" { Instruction::Do }
                rule garbage()
                    = [_]
                rule number() -> usize
                    = n:$(['0'..='9']+) {? n.parse().or(Err("Cannot parse number")) }
            }
        }
        Ok(parser::memory(input)?)
    }
}

impl Memory {
    fn exec(&self) -> usize {
        self.0
            .iter()
            .map(|i| match i {
                Instruction::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum()
    }
    fn exec_with_toggle(&self) -> usize {
        self.0
            .iter()
            .scan(true, |enabled, ins| match ins {
                Instruction::Mul(a, b) if *enabled => Some(a * b),
                Instruction::Mul(_, _) => Some(0),
                Instruction::Dont => {
                    *enabled = false;
                    Some(0)
                }
                Instruction::Do => {
                    *enabled = true;
                    Some(0)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input() -> &'static str {
        // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[fixture]
    fn memory() -> Memory {
        Memory(vec![
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do,
            Instruction::Mul(8, 5),
        ])
    }

    #[rstest]
    fn test_parse_memory(input: &str, memory: Memory) {
        let parsed: Memory = input.parse().unwrap();
        assert_eq!(parsed, memory);
    }

    #[rstest]
    fn test_execute(memory: Memory) {
        assert_eq!(memory.exec(), 161)
    }

    #[rstest]
    fn test_execute_with_toggle(memory: Memory) {
        assert_eq!(memory.exec_with_toggle(), 48)
    }
}
