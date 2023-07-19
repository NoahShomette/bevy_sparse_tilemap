use bevy::app::App;
use bevy::prelude::{Deref, DerefMut, Plugin, Reflect};
use bevy::reflect::FromReflect;
use grid::Grid;
use std::fmt;
use std::iter::StepBy;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Deref, DerefMut, Reflect, FromReflect)]
pub struct GridList<T>(pub Grid<T>);

impl<T: Clone> Clone for GridList<T> {
    fn clone(&self) -> Self {
        GridList(self.0.clone())
    }
}

impl<T> Index<usize> for GridList<T> {
    type Output = [T];

    #[inline]
    fn index(&self, idx: usize) -> &[T] {
        self.0.index(idx)
    }
}

impl<T> IndexMut<usize> for GridList<T> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut [T] {
        self.0.index_mut(idx)
    }
}

impl<T> Index<(usize, usize)> for GridList<T> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &T {
        self.0.index((row, col))
    }
}

impl<T> IndexMut<(usize, usize)> for GridList<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.0.index_mut((row, col))
    }
}

impl<T: fmt::Debug> fmt::Debug for GridList<T> {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Eq> PartialEq for GridList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}

impl<T: Eq> Eq for GridList<T> {}

pub struct GridListRowIter<'a, T> {
    grid: &'a Grid<T>,
    row_index: usize,
}
pub struct GridListColIter<'a, T> {
    grid: &'a Grid<T>,
    col_index: usize,
}

impl<'a, T> Iterator for GridListRowIter<'a, T> {
    type Item = Iter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let rows = self.grid.rows();
        let row_index = self.row_index;

        if !(0..rows).contains(&row_index) {
            return None;
        }

        let row_iter = self.grid.iter_row(row_index);
        self.row_index += 1;
        Some(row_iter)
    }
}

impl<'a, T> Iterator for GridListColIter<'a, T> {
    type Item = StepBy<Iter<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let cols = self.grid.cols();
        let col_index = self.col_index;

        if !(0..cols).contains(&col_index) {
            return None;
        }

        let row_iter = self.grid.iter_col(col_index);
        self.col_index += 1;
        Some(row_iter)
    }
}
