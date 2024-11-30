use std::{fmt::Display, str::FromStr};

pub mod math;

mod input;
pub use input::*;

pub mod graphs;

pub mod grid;
pub mod sparse_grid;

mod regex_helper;
pub use regex_helper::*;

pub trait EvenMoreItertools: Iterator {
    fn sum_usize<I>(self) -> usize
    where
        Self: Iterator<Item = I> + Sized,
        I: TryInto<usize>,
        <I as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    {
        self.map(|i| i.try_into().unwrap()).sum()
    }

    fn sum_u64<I>(self) -> u64
    where
        Self: Iterator<Item = I> + Sized,
        I: TryInto<u64>,
        <I as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
    {
        self.map(|i| i.try_into().unwrap()).sum()
    }

    fn sum_i64<I>(self) -> i64
    where
        Self: Iterator<Item = I> + Sized,
        I: TryInto<i64>,
        <I as std::convert::TryInto<i64>>::Error: std::fmt::Debug,
    {
        self.map(|i| i.try_into().unwrap()).sum()
    }

    fn fold_digits_to_number<N, I>(self) -> N
    where
        Self: Iterator<Item = I> + Sized,
        N: FromStr,
        <N as std::str::FromStr>::Err: std::fmt::Debug,
        I: std::fmt::Display,
    {
        self.map(|i| i.to_string())
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn fold_digits_to_u64<I>(self) -> u64
    where
        Self: Iterator<Item = I> + Sized,
        I: std::fmt::Display,
    {
        self.fold_digits_to_number()
    }
}

impl<T: ?Sized> EvenMoreItertools for T where T: Iterator {}

#[derive(Debug, PartialEq)]
pub struct Solution {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl Solution {
    pub fn copy_to_clipboard(&self) {
        let solution_to_copy = if self.part2.is_some() {
            &self.part2
        } else {
            &self.part1
        };

        if let Some(solution_to_copy) = solution_to_copy {
            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "echo {} | xclip -r -selection clipboard",
                    solution_to_copy
                ))
                .spawn()
                .expect("Failed to copy solution to clipboard");
        }
    }
}

trait PartSolution {
    fn as_part_solution(&self) -> Option<String>;
}

// Rusts's orphan rules prevent a generic implementation because ToString is not implemented
// for (), so do this manually here for all types that might be aoc solutions
macro_rules! impl_part_solution {
    ($($t:ty),*) => {
        $(
            impl PartSolution for $t {
                fn as_part_solution(&self) -> Option<String> {
                    Some(self.to_string())
                }
            }
        )*
    };
}
impl_part_solution!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &str);

impl PartSolution for () {
    fn as_part_solution(&self) -> Option<String> {
        None
    }
}

impl<T: PartSolution> From<T> for Solution {
    fn from(part1: T) -> Self {
        Solution {
            part1: part1.as_part_solution(),
            part2: None,
        }
    }
}

impl<T: PartSolution, U: PartSolution> From<(T, U)> for Solution {
    fn from((part1, part2): (T, U)) -> Self {
        Solution {
            part1: part1.as_part_solution(),
            part2: part2.as_part_solution(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(part1) = &self.part1 {
            writeln!(f, "Solutions:")?;
            writeln!(f, "Part 1: {}", part1)?;
        }
        if let Some(part2) = &self.part2 {
            writeln!(f, "Part 2: {}", part2)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! assert_example {
    ($input:expr, $part1:expr, $part2:expr) => {
        pretty_assertions::assert_eq!(solve(Input::from($input.trim())).into(), Solution::from(($part1, $part2)));
    };
    ($input:expr, $part1:expr) => {
        let solution = solve(Input::from($input.trim())).into();
        pretty_assertions::assert_eq!(solution.part1, Some($part1.to_string()));
    };
}

#[macro_export]
macro_rules! assert_part2 {
    ($input:expr, $part2:expr) => {
        let solution = solve(Input::from($input.trim())).into();
        pretty_assertions::assert_eq!(solution.part2, Some($part2.to_string()));
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn fold_digits() {
        assert_eq!(123, [1, 2, 3].into_iter().fold_digits_to_number());
        assert_eq!(123, ["1", "2", "3"].into_iter().fold_digits_to_number());
    }

    #[test]
    fn collect_from_str() {
        #[derive(aoc_derive::CollectFromStr, PartialEq, Debug)]
        struct ContainsVec(Vec<usize>);

        assert_eq!(Ok(ContainsVec(vec![1, 2, 3])), " 1, 2, 3".parse());

        #[derive(aoc_derive::CollectFromStr, PartialEq, Debug)]
        #[sep = ":"]
        struct ContainsVecColonSep(Vec<i32>);

        assert_eq!(
            Ok(ContainsVecColonSep(vec![-1, 2, -3])),
            " -1: 2: -3".parse()
        );

        #[derive(aoc_derive::CollectFromStr, PartialEq, Debug)]
        struct ContainsHashSet(HashSet<i32>);

        assert_eq!(
            Ok(ContainsHashSet([-1, 2, -3].into_iter().collect())),
            "-1, 2, -3".parse()
        );
    }

    #[test]
    fn hash_map_from_str() {
        #[derive(aoc_derive::HashMapFromStr, PartialEq, Debug)]
        struct ContainsHashMap(HashMap<i32, i32>);

        assert_eq!(
            Ok(ContainsHashMap([(-1, 2), (3, 4)].iter().cloned().collect())),
            "-1: 2, 3: 4".parse()
        );

        #[derive(aoc_derive::HashMapFromStr, PartialEq, Debug)]
        #[sep = ";"]
        #[inner_sep = "=>"]
        struct ContainsHashMapCustomSep(HashMap<i32, i32>);

        assert_eq!(
            Ok(ContainsHashMapCustomSep(
                [(-1, 2), (3, 4)].iter().cloned().collect()
            )),
            "-1 => 2 ; 3 => 4".parse()
        );
    }
}
