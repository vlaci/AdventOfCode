// SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
//
// SPDX-License-Identifier: EUPL-1.2

use color_eyre::eyre::{Report, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let parsed: Lists = INPUT.parse()?;
    let part2 = parsed.cardinality();
    let part1 = parsed.min_distance();

    println!("The answer to the 1st part is {part1}");
    println!("The answer to the 2nd part is {part2}");
    Ok(())
}

static INPUT: &str = include_str!("../input");

#[derive(PartialEq, Debug)]
struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl std::str::FromStr for Lists {
    type Err = Report;
    fn from_str(input: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule lists() -> Lists
                = l:(line() ** "\n") "\n"* { let (left, right) = l.into_iter().unzip(); Lists{left, right} }
                rule line() -> (usize, usize)
                    = a:number() ws() b:number() { (a, b) }
                rule number() -> usize
                = n:$(['0'..='9']+) {? n.parse().or(Err("Cannot parse number")) }
                rule ws()
                    = [' ']+
            }
        }
        Ok(parser::lists(input)?)
    }
}

impl Lists {
    fn min_distance(mut self) -> usize {
        self.left.sort();
        self.right.sort();
        self.left
            .into_iter()
            .zip(self.right)
            .map(|(l, r)| l.abs_diff(r))
            .sum()
    }

    fn cardinality(&self) -> usize {
        self.left.iter().fold(0, |acc, &l| {
            acc + l * self.right.iter().filter(|&r| *r == l).count()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
             3   4
             4   3
             2   5
             1   3
             3   9
             3   3
         "}
    }

    #[fixture]
    fn lists() -> Lists {
        Lists {
            left: vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3],
        }
    }

    #[rstest]
    fn test_parse_lists(input: &str, lists: Lists) {
        let parsed: Lists = input.parse().unwrap();
        assert_eq!(parsed, lists);
    }

    #[rstest]
    fn test_min_distance(lists: Lists) {
        assert_eq!(lists.min_distance(), 11);
    }

    #[rstest]
    fn test_cardinality(lists: Lists) {
        assert_eq!(lists.cardinality(), 31);
    }
}
