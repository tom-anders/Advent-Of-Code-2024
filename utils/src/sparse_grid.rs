use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::math::Vec2D;

#[derive(Debug, PartialEq, Eq, Clone, Default, derive_more::From)]
pub struct SparseGrid<T> {
    data: HashMap<Vec2D, T>,
}

impl<T: Hash, V: Into<Vec2D>> FromIterator<(V, T)> for SparseGrid<T> {
    fn from_iter<I: IntoIterator<Item = (V, T)>>(iter: I) -> Self {
        Self::from(
            iter.into_iter()
                .map(|(pos, val)| (pos.into(), val))
                .collect::<HashMap<_,_>>(),
        )
    }
}

impl<T> Deref for SparseGrid<T> {
    type Target = HashMap<Vec2D, T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for SparseGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> SparseGrid<T>
where
    T: Hash,
{
    pub fn new() -> Self {
        SparseGrid { data: HashMap::new() }
    }

    pub fn get(&self, pos: impl Into<Vec2D>) -> Option<&T> {
        self.data.get(&pos.into())
    }

    pub fn get_mut(&mut self, pos: impl Into<Vec2D>) -> Option<&mut T> {
        self.data.get_mut(&pos.into())
    }

    pub fn orthogonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = (&Vec2D, &T)> + 'a {
        pos.orthogonal_neighbors()
            .flat_map(|pos| self.get(pos))
            .map(move |val| (pos, val))
    }

    pub fn diagonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = (&Vec2D, &T)> + 'a {
        pos.diagonal_neighbors()
            .flat_map(|pos| self.get(pos))
            .map(move |val| (pos, val))
    }

    pub fn all_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = (&Vec2D, &T)> + 'a {
        pos.all_neighbors()
            .flat_map(|pos| self.get(pos))
            .map(move |val| (pos, val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let grid: SparseGrid<_> = [((0, 0), "a"), ((1, 0), "b"), ((0, 1), "c"), ((1, 1), "d")]
            .into_iter()
            .collect();

        assert_eq!(grid.get((0, 0)), Some(&"a"));
        assert_eq!(grid.get((1, 0)), Some(&"b"));
        assert_eq!(grid.get((0, 1)), Some(&"c"));
        assert_eq!(grid.get((1, 1)), Some(&"d"));
        assert_eq!(grid.get((2, 0)), None);
        assert_eq!(grid.get((0, 2)), None);
        assert_eq!(grid.get((2, 2)), None);
    }
}
