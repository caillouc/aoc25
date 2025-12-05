#![allow(dead_code)]
use super::direction::Direction;
use std::{
    fmt,
    ops::{Add, Sub},
    usize, vec,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    pub fn is_valid(&self, x_max: usize, y_max: usize) -> bool {
        self.x >= 0 && self.x < x_max as i32 && self.y >= 0 && self.y < y_max as i32
    }
    pub fn x_i32(&self) -> i32 {
        self.x
    }
    pub fn y_i32(&self) -> i32 {
        self.y
    }
    pub fn x(&self) -> usize {
        if self.x < 0 {
            return 0;
        }
        self.x as usize
    }
    pub fn y(&self) -> usize {
        if self.y < 0 {
            return 0;
        }
        self.y as usize
    }

    fn to(&self, dir: Direction, max_x: i32, max_y: i32) -> Option<Self> {
        let new_pos = *self + dir.pos();
        if new_pos.x >= 0 && new_pos.x < max_x && new_pos.y >= 0 && new_pos.y < max_y
        {
            return Some(new_pos);
        }
        None
    }

    fn toto(
        &self,
        dir1: Direction,
        dir2: Direction,
        max_x: i32,
        max_y: i32,
    ) -> Option<Self> {
        let new_pos = *self + dir1.pos() + dir2.pos();
        if new_pos.x >= 0 && new_pos.x < max_x && new_pos.y >= 0 && new_pos.y < max_y
        {
            return Some(new_pos);
        }
        None
    }

    pub fn adjacent_pos(&self, max_x: usize, max_y: usize) -> Vec<Option<Self>> {
        vec![
            self.up_left(),
            self.up(),
            self.up_rigth(max_x),
            self.left(),
            self.rigth(max_x),
            self.down_left(max_y),
            self.down(max_y),
            self.down_rigth(max_x, max_y),
        ]
    }

    pub fn left(&self) -> Option<Self> {
        self.to(Direction::Left, i32::MAX, i32::MAX)
    }
    pub fn rigth(&self, max_x: usize) -> Option<Self> {
        self.to(Direction::Rigth, max_x as i32, i32::MAX)
    }
    pub fn up(&self) -> Option<Self> {
        self.to(Direction::Up, i32::MAX, i32::MAX)
    }
    pub fn down(&self, max_y: usize) -> Option<Self> {
        self.to(Direction::Down, i32::MAX, max_y as i32)
    }
    pub fn up_left(&self) -> Option<Self> {
        self.toto(Direction::Up, Direction::Left, i32::MAX, i32::MAX)
    }
    pub fn down_left(&self, max_y: usize) -> Option<Self> {
        self.toto(Direction::Down, Direction::Left, i32::MAX, max_y as i32)
    }
    pub fn up_rigth(&self, max_x: usize) -> Option<Self> {
        self.toto(Direction::Up, Direction::Rigth, max_x as i32, i32::MAX)
    }
    pub fn down_rigth(&self, max_x: usize, max_y: usize) -> Option<Self> {
        self.toto(Direction::Down, Direction::Rigth, max_x as i32, max_y as i32)
    }
}
