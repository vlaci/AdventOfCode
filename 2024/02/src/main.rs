// SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
//
// SPDX-License-Identifier: EUPL-1.2

use color_eyre::eyre::{Report, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let parsed: Reports = INPUT.parse()?;
    let part1 = parsed.count_safe();
    println!("The answer to the 1st part is {part1}");

    let part2 = parsed.count_safe_dampened();
    println!("The answer to the 2nd part is {part2}");
    Ok(())
}

static INPUT: &str = include_str!("../input");

#[derive(PartialEq, Debug)]
struct Reports(Vec<Vec<usize>>);

impl std::str::FromStr for Reports {
    type Err = Report;
    fn from_str(input: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule reports() -> Reports
                    = l:(report() ** "\n") "\n"* { Reports(l) }
                rule report() -> Vec<usize>
                    = r:(number() ++ " ") { r }
                rule number() -> usize
                    = n:$(['0'..='9']+) {? n.parse().or(Err("Cannot parse number")) }
            }
        }
        Ok(parser::reports(input)?)
    }
}

impl Reports {
    fn count_safe(&self) -> usize {
        self.0
            .iter()
            .filter(|&report| {
                let deltas: Vec<isize> = report
                    .iter()
                    .zip(report.iter().skip(1))
                    .map(|(&a, &b)| a as isize - b as isize)
                    .collect();
                deltas.iter().all(|delta| (1..=3).contains(delta))
                    || deltas.iter().all(|delta| (-3..=-1).contains(delta))
            })
            .count()
    }
    fn count_safe_dampened(&self) -> usize {
        self.0
            .iter()
            .filter(|&report| {
                for skip in 0..report.len() {
                    let mut report = report.clone();
                    report.remove(skip);
                    let deltas: Vec<isize> = report
                        .iter()
                        .zip(report.iter().skip(1))
                        .map(|(&a, &b)| a as isize - b as isize)
                        .collect();

                    if deltas.iter().all(|delta| (1..=3).contains(delta))
                        || deltas.iter().all(|delta| (-3..=-1).contains(delta))
                    {
                        return true;
                    }
                }
                false
            })
            .count()
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
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
         "}
    }

    #[fixture]
    fn reports() -> Reports {
        Reports(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ])
    }

    #[rstest]
    fn test_parse_reports(input: &str, reports: Reports) {
        let parsed: Reports = input.parse().unwrap();
        assert_eq!(parsed, reports);
    }

    #[rstest]
    fn test_count_safe(reports: Reports) {
        assert_eq!(reports.count_safe(), 2)
    }

    #[rstest]
    fn test_count_safe_dampened(reports: Reports) {
        assert_eq!(reports.count_safe_dampened(), 4)
    }
}
