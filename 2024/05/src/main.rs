// SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::{HashMap, HashSet};

use color_eyre::eyre::{Report, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let parsed: PrintQueue = INPUT.parse()?;
    let part1 = parsed.sum_ordered();
    let part2 = parsed.sum_unordered();

    println!("The answer to the 1st part is {part1}");
    println!("The answer to the 2nd part is {part2}");
    Ok(())
}

static INPUT: &str = include_str!("../input");

#[derive(PartialEq, Debug)]
struct PrintQueue {
    rules: HashMap<usize, Vec<usize>>,
    pages: Vec<Vec<usize>>,
}

impl std::str::FromStr for PrintQueue {
    type Err = Report;
    fn from_str(input: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule queue() -> PrintQueue
                = r:(order() ++ "\n") "\n\n" pages:(page() ++ "\n") "\n"* {
                    let mut rules = HashMap::new();
                    for (a, b) in r.into_iter() {
                        rules.entry(a).or_insert_with(Vec::new).push(b);
                    }
                    PrintQueue{rules, pages}
                }
                rule order() -> (usize, usize)
                    = a:number() "|" b:number() { (a, b) }
                rule page() -> Vec<usize>
                    = n:(number() ++ ",")
                rule number() -> usize
                    = n:$(['0'..='9']+) {? n.parse().or(Err("Cannot parse number")) }
            }
        }
        Ok(parser::queue(input)?)
    }
}

impl PrintQueue {
    fn is_ordered(&self, pages: &[usize]) -> bool {
        let mut passed = vec![];

        for &page in pages {
            if let Some(after) = &self.rules.get(&page) {
                for p in passed.iter() {
                    if after.contains(p) {
                        return false;
                    }
                }
            }
            passed.push(page);
        }
        true
    }

    fn sum_ordered(&self) -> usize {
        let mut rv = 0;
        for p in self.pages.iter().filter(|&p| self.is_ordered(p)) {
            rv += p[p.len() / 2];
        }
        rv
    }

    fn sort(&self, pages: &[usize]) -> Vec<usize> {
        let mut edges: HashSet<_> = self
            .rules
            .iter()
            .filter(|&(k, _)| pages.contains(k))
            .flat_map(|(k, vals)| vals.iter().filter(|&v| pages.contains(v)).map(|v| (*k, *v)))
            .collect();
        let mut nodes: Vec<usize> = pages
            .iter()
            .filter(|&&n| !edges.iter().any(|(_, b)| *b == n))
            .cloned()
            .collect();
        // Kahn's algorythm
        let mut rv = vec![];

        while let Some(n) = nodes.pop() {
            rv.push(n);
            for e in edges.clone() {
                if e.0 == n {
                    edges.remove(&e);
                }
                if !edges.iter().any(|d| d.1 == e.1) {
                    nodes.push(e.1);
                }
            }
        }
        rv
    }

    fn sum_unordered(&self) -> usize {
        let mut rv = 0;
        for p in self.pages.iter().filter(|&p| !self.is_ordered(p)) {
            let ordered = self.sort(p);
            rv += ordered[ordered.len() / 2];
        }
        rv
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
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
         "}
    }

    #[fixture]
    fn print_queue() -> PrintQueue {
        PrintQueue {
            rules: HashMap::from([
                (47, vec![53, 13, 61, 29]),
                (97, vec![13, 61, 47, 29, 53, 75]),
                (75, vec![29, 53, 47, 61, 13]),
                (61, vec![13, 53, 29]),
                (29, vec![13]),
                (53, vec![29, 13]),
            ]),
            pages: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        }
    }

    #[rstest]
    fn test_parse(input: &str, print_queue: PrintQueue) {
        let parsed: PrintQueue = input.parse().unwrap();
        assert_eq!(parsed, print_queue);
    }

    #[rstest]
    fn test_is_ordered(print_queue: PrintQueue) {
        assert!(print_queue.is_ordered(&print_queue.pages[0]));
        assert!(print_queue.is_ordered(&print_queue.pages[1]));
        assert!(print_queue.is_ordered(&print_queue.pages[2]));
        assert!(!print_queue.is_ordered(&print_queue.pages[3]));
        assert!(!print_queue.is_ordered(&print_queue.pages[4]));
        assert!(!print_queue.is_ordered(&print_queue.pages[5]));
    }
    #[rstest]
    fn test_sum_ordered(print_queue: PrintQueue) {
        assert_eq!(print_queue.sum_ordered(), 143);
    }
    #[rstest]
    fn test_sort(print_queue: PrintQueue) {
        assert_eq!(
            print_queue.sort(&print_queue.pages[3]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(print_queue.sort(&print_queue.pages[4]), vec![61, 29, 13]);
        assert_eq!(
            print_queue.sort(&print_queue.pages[5]),
            vec![97, 75, 47, 29, 13]
        );
    }
    #[rstest]
    fn test_sum_unordered(print_queue: PrintQueue) {
        assert_eq!(print_queue.sum_unordered(), 123);
    }
}
