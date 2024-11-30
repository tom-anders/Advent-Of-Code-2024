use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::{graphs, math::Vec2D};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T, InnerIter> FromIterator<InnerIter> for Grid<T>
where
    InnerIter: IntoIterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = InnerIter>>(iter: I) -> Self {
        iter.into()
    }
}

impl<T, InnerIter: IntoIterator<Item = T>, Iter: IntoIterator<Item = InnerIter>> From<Iter>
    for Grid<T>
{
    fn from(iter: Iter) -> Self {
        let grid = Grid {
            data: iter.into_iter().map(|iter| iter.into_iter().collect_vec()).collect_vec(),
        };
        assert!(grid.data.iter().all(|row| row.len() == grid.data[0].len()));
        grid
    }
}

/// Never do this in production, but in AoC implicit conversion between integer types is very
/// helpful, we assume there's never any overflow
pub trait UnwrapIntoUsize {
    fn unwrap_usize(self) -> usize;
}

impl<T> UnwrapIntoUsize for T
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn unwrap_usize(self) -> usize {
        self.try_into().unwrap()
    }
}

impl<T, Pos> Index<Pos> for Grid<T>
where
    Pos: Into<Vec2D>,
{
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        self.get(pos.into()).unwrap()
    }
}

impl<T, Pos> IndexMut<Pos> for Grid<T>
where
    Pos: Into<Vec2D>,
{
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        self.get_mut(pos.into()).unwrap()
    }
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        assert!(data.iter().all(|row| row.len() == data[0].len()));
        Grid { data }
    }

    pub fn inner(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.data
    }

    pub fn get_wrapping(&self, pos: impl Into<Vec2D>) -> &T {
        let mut pos = pos.into();

        while pos.x < 0 {
            pos.x += self.num_cols() as i64;
        }

        while pos.y < 0 {
            pos.y += self.num_rows() as i64;
        }

        &self[(pos.x % self.num_cols() as i64, pos.y % self.num_cols() as i64)]
    }

    pub fn get(&self, pos: impl Into<Vec2D>) -> Option<&T> {
        let pos = pos.into();
        self.contains(&pos).then(|| &self.data[pos.y as usize][pos.x as usize])
    }

    pub fn get_mut(&mut self, pos: impl Into<Vec2D>) -> Option<&mut T> {
        let pos = pos.into();
        self.contains(&pos).then(|| &mut self.data[pos.y as usize][pos.x as usize])
    }

    pub fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.num_cols() as i64 && pos.y < self.num_rows() as i64
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn row(
        &self,
        row: impl UnwrapIntoUsize,
    ) -> impl DoubleEndedIterator<Item = (Vec2D, &T)> + '_ {
        let row = row.unwrap_usize();
        self.data[row].iter().enumerate().map(move |(col, item)| ((col, row).into(), item))
    }

    pub fn row_values(&self, row: impl UnwrapIntoUsize) -> impl Iterator<Item = &T> + '_ {
        self.row(row.unwrap_usize()).map(|(_, item)| item)
    }

    pub fn rotate_row_left(&mut self, row: usize, mid: usize) {
        self.data[row].rotate_left(mid);
    }

    pub fn rotate_row_right(&mut self, row: usize, mid: usize) {
        self.data[row].rotate_right(mid);
    }

    pub fn rows(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = (Vec2D, &T)>> + '_ {
        (0..self.num_rows()).map(move |row| self.row(row))
    }

    pub fn num_cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn col(&self, col: impl UnwrapIntoUsize) -> ColIter<'_, T> {
        ColIter::new(self, col.unwrap_usize())
    }

    pub fn col_values(&self, col: impl UnwrapIntoUsize) -> impl Iterator<Item = &T> + '_ {
        self.col(col).map(|(_, item)| item)
    }

    pub fn cols(&self) -> impl DoubleEndedIterator<Item = ColIter<'_, T>> + '_ {
        (0..self.num_cols()).map(move |col| self.col(col))
    }

    pub fn coordinates_col_major(&self) -> impl DoubleEndedIterator<Item = Vec2D> + '_ {
        (0..self.num_cols())
            .flat_map(|x| (0..self.num_rows()).map(move |y| (x, y)))
            .map(Vec2D::from)
    }

    pub fn coordinates_row_major(&self) -> impl DoubleEndedIterator<Item = Vec2D> + '_ {
        (0..self.num_rows())
            .flat_map(|y| (0..self.num_cols()).map(move |x| (x, y)))
            .map(Vec2D::from)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Vec2D, &T)> + '_ {
        self.coordinates_row_major().map(move |pos| (pos, &self[pos]))
    }

    pub fn orthogonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = Vec2D> + 'a {
        pos.orthogonal_neighbors().filter(move |neighbor| self.contains(neighbor))
    }

    pub fn diagonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = Vec2D> + 'a {
        pos.diagonal_neighbors().filter(move |neighbor| self.contains(neighbor))
    }

    pub fn all_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        pos.all_neighbors().filter(move |neighbor| self.contains(neighbor))
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn with_value(val: T, num_rows: usize, num_cols: usize) -> Self {
        Grid { data: vec![vec![val; num_cols]; num_rows] }
    }

    pub fn pad_edges(self, with: T) -> Self {
        let mut grid = Grid::with_value(with, self.num_rows() + 2, self.num_cols() + 2);
        for (pos, item) in self.iter() {
            grid[pos + (1, 1)] = item.clone();
        }
        grid
    }

    pub fn swap(&mut self, lhs: Vec2D, rhs: Vec2D) {
        // Can't use std::mem::swap here because it would require two mutable references to self
        let tmp = self[lhs].clone();
        self[lhs] = self[rhs].clone();
        self[rhs] = tmp
    }

    fn rotate_col(&mut self, col: usize, mid: usize, up: bool) {
        let mut new_col = self.col(col).map(|(_, item)| item).cloned().collect_vec();
        if up {
            new_col.rotate_left(mid);
        } else {
            new_col.rotate_right(mid);
        }

        for (row, item) in new_col.into_iter().enumerate() {
            self.data[row][col] = item;
        }
    }

    pub fn rotate_col_up(&mut self, col: usize, mid: usize) {
        self.rotate_col(col, mid, true);
    }

    pub fn rotate_col_down(&mut self, col: usize, mid: usize) {
        self.rotate_col(col, mid, false);
    }
}

pub struct ColIter<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    row_back: usize,
    col: usize,
}

impl<'a, T> ColIter<'a, T> {
    fn new(grid: &'a Grid<T>, col: usize) -> Self {
        ColIter { grid, row: 0, row_back: grid.num_rows(), col }
    }
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = (Vec2D, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_back {
            return None;
        }
        let pos = (self.col, self.row).into();
        let item = (pos, &self.grid[pos]);
        self.row += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.row_back - self.row;
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for ColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row_back == self.row {
            return None;
        }
        self.row_back -= 1;
        let pos = (self.col, self.row_back).into();
        let item = (pos, &self.grid[pos]);
        Some(item)
    }
}

pub trait UnweightedGrid {
    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a;
}

pub trait WeightedGrid {
    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a;

    fn cost(&self, from: Vec2D, to: Vec2D) -> graphs::Cost;
}

impl<T> graphs::WeightedGraph for Grid<T>
where
    Grid<T>: WeightedGrid,
{
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, graphs::Cost)> + 'a {
        assert!(self.contains(node));
        WeightedGrid::neighbors(self, node)
            .map(move |neighbor| (neighbor, self.cost(*node, neighbor)))
    }
}

impl<T> graphs::UnweightedGraph for Grid<T>
where
    Grid<T>: UnweightedGrid,
{
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = Self::Node> + 'a {
        assert!(self.contains(node));
        UnweightedGrid::neighbors(self, node)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn iter_row_and_col() {
        let grid: Grid<_> = [[1, 2, 3], [4, 5, 6]].into();

        assert_eq!(
            grid.row(0).collect_vec(),
            vec![(Vec2D::new(0, 0), &1), (Vec2D::new(1, 0), &2), (Vec2D::new(2, 0), &3)]
        );

        assert_eq!(
            grid.row(0).rev().collect_vec(),
            vec![(Vec2D::new(2, 0), &3), (Vec2D::new(1, 0), &2), (Vec2D::new(0, 0), &1)]
        );

        assert_eq!(
            grid.row(1).collect_vec(),
            vec![(Vec2D::new(0, 1), &4), (Vec2D::new(1, 1), &5), (Vec2D::new(2, 1), &6)]
        );

        assert_eq!(
            grid.rows().map(|row| row.collect_vec()).collect_vec(),
            vec![
                vec![(Vec2D::new(0, 0), &1), (Vec2D::new(1, 0), &2), (Vec2D::new(2, 0), &3)],
                vec![(Vec2D::new(0, 1), &4), (Vec2D::new(1, 1), &5), (Vec2D::new(2, 1), &6)],
            ]
        );

        assert_eq!(grid.col(0).collect_vec(), vec![(Vec2D::new(0, 0), &1), (Vec2D::new(0, 1), &4)]);

        assert_eq!(
            grid.col(0).rev().collect_vec(),
            vec![(Vec2D::new(0, 1), &4), (Vec2D::new(0, 0), &1),]
        );

        let mut col0_iter = grid.col(0);
        assert_eq!(col0_iter.size_hint(), (2, Some(2)));
        assert_eq!(col0_iter.next(), Some((Vec2D::new(0, 0), &1)));
        assert_eq!(col0_iter.size_hint(), (1, Some(1)));
        assert_eq!(col0_iter.next_back(), Some((Vec2D::new(0, 1), &4)));
        assert_eq!(col0_iter.next(), None);
        assert_eq!(col0_iter.next_back(), None);

        assert_eq!(grid.col(1).collect_vec(), vec![(Vec2D::new(1, 0), &2), (Vec2D::new(1, 1), &5)]);

        assert_eq!(
            grid.cols().map(|col| col.collect_vec()).collect_vec(),
            vec![
                vec![(Vec2D::new(0, 0), &1), (Vec2D::new(0, 1), &4)],
                vec![(Vec2D::new(1, 0), &2), (Vec2D::new(1, 1), &5)],
                vec![(Vec2D::new(2, 0), &3), (Vec2D::new(2, 1), &6)],
            ]
        );
    }

    #[test]
    fn rotate_rows_and_cols() {
        let mut grid: Grid<_> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].into();

        grid.rotate_row_left(0, 1);
        assert_eq!(grid.data, [[2, 3, 1], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_row_right(0, 1);
        assert_eq!(grid.data, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_row_left(1, 2);
        assert_eq!(grid.data, [[1, 2, 3], [6, 4, 5], [7, 8, 9]]);

        grid.rotate_row_right(1, 2);
        assert_eq!(grid.data, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_col_up(0, 1);
        assert_eq!(grid.data, [[4, 2, 3], [7, 5, 6], [1, 8, 9]]);

        grid.rotate_col_down(0, 2);
        assert_eq!(grid.data, [[7, 2, 3], [1, 5, 6], [4, 8, 9]]);
    }

    #[test]
    fn pad_edges() {
        let grid: Grid<_> = [[1, 2, 3], [4, 5, 6]].into();
        let padded = grid.pad_edges(0);
        assert_eq!(
            padded.data,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 2, 3, 0],
                vec![0, 4, 5, 6, 0],
                vec![0, 0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn get_wrapping() {
        let grid: Grid<_> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(grid.get_wrapping((0, 0)), &1);
        assert_eq!(grid.get_wrapping((-1, 0)), &3);
        assert_eq!(grid.get_wrapping((-3, 0)), &1);
        assert_eq!(grid.get_wrapping((-6, -2)), &1);
        assert_eq!(grid.get_wrapping((-1, 1)), &6);
        assert_eq!(grid.get_wrapping((-1, -1)), &6);
    }

    #[test]
    fn iter() {
        let grid: Grid<_> = [[1, 2, 3], [4, 5, 6]].into();

        assert_eq!(
            grid.coordinates_row_major().collect_vec(),
            vec![
                Vec2D::new(0, 0),
                Vec2D::new(1, 0),
                Vec2D::new(2, 0),
                Vec2D::new(0, 1),
                Vec2D::new(1, 1),
                Vec2D::new(2, 1),
            ]
        );
        assert_eq!(
            grid.coordinates_col_major().collect_vec(),
            vec![
                Vec2D::new(0, 0),
                Vec2D::new(0, 1),
                Vec2D::new(1, 0),
                Vec2D::new(1, 1),
                Vec2D::new(2, 0),
                Vec2D::new(2, 1),
            ]
        );

        assert_eq!(
            grid.iter().collect_vec(),
            vec![
                (Vec2D::new(0, 0), &1),
                (Vec2D::new(1, 0), &2),
                (Vec2D::new(2, 0), &3),
                (Vec2D::new(0, 1), &4),
                (Vec2D::new(1, 1), &5),
                (Vec2D::new(2, 1), &6)
            ],
        )
    }
}
