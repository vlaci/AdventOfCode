// SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use color_eyre::eyre::{Report, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let parsed: WordSearch = INPUT.parse()?;
    let part1 = parsed.count(&*XMAS);
    println!("The answer to the 1st part is {part1}");

    let part2 = parsed.count(&*MAS_AS_X);
    println!("The answer to the 2nd part is {part2}");
    Ok(())
}

static INPUT: &str = include_str!("../input");

struct WordSearch(HashMap<(usize, usize), u8>);

impl FromStr for WordSearch {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut map = HashMap::new();
        s.lines()
            .map(|l| l.as_bytes().iter().enumerate())
            .enumerate()
            .for_each(|(row, coch)| {
                for (col, ch) in coch {
                    map.insert((row, col), *ch);
                }
            });
        Ok(Self(map))
    }
}

struct Pattern {
    chars: Vec<u8>,
    paths: Vec<Vec<(i32, i32)>>,
}

static XMAS: LazyLock<Pattern> = LazyLock::new(|| {
    //     -3 -2 -1  0  1  2  3
    //
    // -3   S        S        S
    // -2      A     A     A
    // -1         M  M  M
    //  0   S  A  M  X  M  A  S
    //  1         M  M  M
    //  2      A     A     A
    //  3   S        S        S

    let chars = b"XMAS".to_vec();

    let paths = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
    ];

    Pattern { chars, paths }
});

static MAS_AS_X: LazyLock<Pattern> = LazyLock::new(|| {
    //      -1  0  1   -1  0  1
    // -1    M     S    M     M
    //  0       A          A
    //  1    M     S    S     S
    //
    //
    // -1    S     M    S     S
    //  0       A          A
    //  1    S     M    M     M

    let chars = b"MMASS".to_vec();

    let paths = vec![
        vec![(-1, -1), (1, -1), (0, 0), (1, 1), (-1, 1)],
        vec![(-1, 1), (1, 1), (0, 0), (1, -1), (-1, -1)],
        vec![(-1, -1), (-1, 1), (0, 0), (1, -1), (1, 1)],
        vec![(1, -1), (1, 1), (0, 0), (-1, -1), (-1, 1)],
    ];

    Pattern { chars, paths }
});

impl WordSearch {
    fn count_at(&self, (x, y): &(usize, usize), pattern: &Pattern) -> usize {
        let mut rv = 0;
        for path in pattern.paths.iter() {
            let mut found = true;
            for ((dx, dy), expected) in path.iter().zip(&pattern.chars) {
                if let Some(ch) = self
                    .0
                    .get(&((*x as i32 + dx) as usize, (*y as i32 + dy) as usize))
                {
                    if ch != expected {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                rv += 1;
            }
        }
        rv
    }

    fn count(&self, pattern: &Pattern) -> usize {
        let mut rv = 0;
        for coord in self.0.keys() {
            rv += self.count_at(coord, pattern);
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
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
         "}
    }

    #[rstest]
    fn test_count_xmas(input: &str) {
        assert_eq!(input.parse::<WordSearch>().unwrap().count(&*XMAS), 18);
    }

    #[rstest]
    fn test_count_masx(input: &str) {
        assert_eq!(input.parse::<WordSearch>().unwrap().count(&*MAS_AS_X), 9);
    }
}
