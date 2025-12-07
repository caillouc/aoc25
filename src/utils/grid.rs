#![allow(dead_code)]
use std::slice::Iter;
use super::position::Position;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    x_max: usize,
    y_max: usize,
}
pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<T> Grid<T> {
    pub fn from(data: Vec<Vec<T>>) -> Self {
        let x_max = data[0].len();
        let y_max = data.len();
        Self { data, x_max, y_max }
    }

    pub fn get_from_pos(&self, pos: Position) -> Option<&T> {
        match self.data.get(pos.y()) {
            Some(row) => row.get(pos.x()),
            None => None,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.get_from_pos(Position::from(x, y))
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator { grid: self, x: 0, y: 0 }
    }

    pub fn contains_pos(&self, pos: Position) -> bool {
        pos.is_valid(self.x_max, self.y_max)
    }

    pub fn first_row(&self) -> Option<&Vec<T>> {
        self.data.get(0)
    }

    pub fn rows(&self) -> Iter<Vec<T>> {
        self.data.iter()
    }
}


impl<'a, T> Iterator for GridIterator<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.x < self.grid.x_max {
            self.x += 1;
        } else {
            self.x = 0;
            if self.y < self.grid.y_max {
                self.y += 1;
            } else {
                return None;
            }
        }
        self.grid.get(self.x, self.y)
    }
}
