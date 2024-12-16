use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

pub type Point2<T> = Vec2<T>;
