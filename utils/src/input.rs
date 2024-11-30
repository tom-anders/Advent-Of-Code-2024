use std::str::{Chars, FromStr};

use itertools::Itertools;

use crate::{grid::Grid, math::Vec2D, sparse_grid::SparseGrid};

pub struct Input {
    pub raw: String,
}

impl<T> From<T> for Input where T: ToString {
    fn from(value: T) -> Self {
        Self { raw: value.to_string().trim().to_string() }
    }
}

impl Input {
    pub fn new(input_file: &str) -> Input {
        Input { raw: std::fs::read_to_string(input_file).unwrap() }
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> + '_ {
        self.raw.lines()
    }

    pub fn parse_blocks<T: FromStr>(&self) -> Vec<Vec<T>> {
        self.lines()
            .collect_vec()
            .split(|line| line.is_empty())
            .map(|lines| lines.iter().map(|line| line.parse().ok().unwrap()).collect())
            .collect()
    }

    pub fn blocks(&self) -> impl Iterator<Item = &str> {
        self.raw.split("\n\n")
    }

    pub fn split_and_parse<T: FromStr>(&self, sep: &'static str) -> impl Iterator<Item = T> + '_
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.raw.split(sep).map(|s| s.trim().parse().unwrap())
    }

    pub fn numbers(&self, sep: &'static str) -> impl Iterator<Item = i64> + '_ {
        self.split_and_parse(sep)
    }

    pub fn parse_lines<T: FromStr>(&self) -> impl Iterator<Item = T> + '_ + Clone
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.raw.lines().map(|line| line.parse::<T>().unwrap())
    }

    pub fn parse_grid<T: FromStr>(&self, sep: &str) -> Grid<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.lines().map(|line| line.split(sep).map(|s| s.parse().unwrap()).collect_vec()).into()
    }

    pub fn parse_sparse_grid<T: FromStr>(&self, sep: &str) -> SparseGrid<T>
    where
        T: std::hash::Hash,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.split(sep)
                    .enumerate()
                    .map(move |(x, s)| (Vec2D::from((x, y)), s.parse().unwrap()))
            })
            .collect()
    }

    pub fn number_grid(&self) -> Grid<usize> {
        self.parse_grid_from_characters()
    }

    pub fn char_grid(&self) -> Grid<char> {
        self.lines().map(str::chars).collect()
    }

    pub fn char_sparse_grid(&self) -> SparseGrid<char> {
        self.char_grid().iter().map(|(pos, c)| (pos, *c)).collect()
    }

    pub fn parse_grid_from_characters<T: FromStr>(&self) -> Grid<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.lines().map(|line| line.chars().map(|c| c.to_string().parse().unwrap())).collect()
    }

    pub fn chars(&self) -> impl Iterator<Item = Chars<'_>> + '_ {
        self.lines().map(|line| line.chars())
    }

    pub fn get_line(&self, pos: usize) -> &str {
        self.raw.lines().nth(pos).unwrap()
    }
}

pub trait ParseInput<T> {
    fn parse_lines(input: &Input) -> impl Iterator<Item = T> + Clone;
    fn split_and_parse(input: &Input, sep: &'static str) -> impl Iterator<Item = T>;
}

impl<T> ParseInput<T> for T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn parse_lines(input: &Input) -> impl Iterator<Item = T> + Clone {
        input.parse_lines()
    }

    fn split_and_parse(input: &Input, sep: &'static str) -> impl Iterator<Item = T> {
        input.split_and_parse(sep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let input = Input { raw: "a\nb\n\nc\n".to_string() };
        assert_eq!(input.lines().collect_vec(), vec!["a", "b", "", "c"]);

        assert_eq!(input.get_line(0), "a");
        assert_eq!(input.get_line(1), "b");
        assert_eq!(input.get_line(2), "");
        assert_eq!(input.get_line(3), "c");
    }

    #[test]
    fn parse_blocks() {
        let input = Input { raw: "1\n2\n\n3\n".to_string() };
        assert_eq!(vec![vec![1, 2], vec![3]], input.parse_blocks());
    }

    #[test]
    fn numbers() {
        let input = Input { raw: "1,2,3".to_string() };
        assert_eq!(vec![1, 2, 3], input.numbers(",").collect_vec());
    }

    #[test]
    fn split_and_parse() {
        let input = Input { raw: "1<<2<<+3".to_string() };
        assert_eq!(vec![1, 2, 3], input.split_and_parse("<<").collect_vec());
    }

    #[test]
    fn parse_lines() {
        let input = Input { raw: "1\n2\n123\n".to_string() };
        assert_eq!(vec![1, 2, 123], input.parse_lines().collect_vec());
    }

    #[test]
    fn chars() {
        let input = Input { raw: "ab\nc\n".to_string() };
        assert_eq!(
            vec![vec!['a', 'b'], vec!['c']],
            input.chars().map(Itertools::collect_vec).collect_vec()
        );
    }

    #[test]
    fn parse_vec2() {
        use crate::math::Vec2D;
        let input = Input { raw: "(1, 2)\n[3, 4]\n".to_string() };
        assert_eq!(
            vec![Vec2D::new(1, 2), Vec2D::new(3, 4)],
            Vec2D::parse_lines(&input).collect_vec()
        );
    }

    #[test]
    fn parse_grid() {
        let input = Input { raw: "a,bb,c\ndd,e,ff\n".to_string() };
        assert_eq!(
            Grid::from(vec![
                vec!["a".to_string(), "bb".to_string(), "c".to_string()],
                vec!["dd".to_string(), "e".to_string(), "ff".to_string()]
            ]),
            input.parse_grid(",")
        );
    }

    #[test]
    fn parse_sparse_grid() {
        let input = Input { raw: "a,bb\ndd,e,ff\nx".to_string() };
        assert_eq!(
            SparseGrid::from_iter(
                [
                    (Vec2D::new(0, 0), "a".to_string()),
                    (Vec2D::new(1, 0), "bb".to_string()),
                    (Vec2D::new(0, 1), "dd".to_string()),
                    (Vec2D::new(1, 1), "e".to_string()),
                    (Vec2D::new(2, 1), "ff".to_string()),
                    (Vec2D::new(0, 2), "x".to_string()),
                ]
                .into_iter()
            ),
            input.parse_sparse_grid(",")
        );
    }

    #[test]
    fn parse_grid_from_characters() {
        let input = Input { raw: "1234\n4567\n".to_string() };
        assert_eq!(Grid::from([[1, 2, 3, 4], [4, 5, 6, 7]]), input.parse_grid_from_characters())
    }
}
