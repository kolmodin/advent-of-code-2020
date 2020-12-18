use std::iter::successors;
use std::ops::Add;
use std::ops::Sub;

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Self {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Self {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
    pub fn origo() -> Pos {
        Pos { x: 0, y: 0 }
    }

    pub fn scale(&self, scale: i32) -> Pos {
        Pos {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn above(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn below(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn neighbors(&self) -> Vec<Pos> {
        vec![
            self.above(),
            self.left(),
            self.right(),
            self.below(),
            self.above().left(),
            self.above().right(),
            self.below().left(),
            self.below().right(),
        ]
    }

    pub fn cardinal(&self) -> Vec<Pos> {
        vec![self.above(), self.left(), self.right(), self.below()]
    }

    pub fn rotate_left(&self) -> Pos {
        Pos {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn rotate_right(&self) -> Pos {
        Pos {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_180(&self) -> Pos {
        Pos {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn repeat_delta_forever(self, delta: Pos) -> impl Iterator<Item = Pos> {
        successors(Some(self), move |acc| Some(*acc + delta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(Pos::origo().above().rotate_right(), Pos::origo().right());
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(Pos::origo().left().rotate_right(), Pos::origo().above());
    }

    #[test]
    fn test_rotate_180() {
        assert_eq!(Pos::origo().left().rotate_180(), Pos::origo().right());
        assert_eq!(Pos::origo().above().rotate_180(), Pos::origo().below());
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Pos::origo().above().above() + Pos::origo().left(),
            Pos::origo().left().above().above()
        );
    }
    #[test]
    fn test_sub() {
        assert_eq!(
            Pos::origo().above().above() - Pos::origo().left(),
            Pos::origo().right().above().above()
        );
    }
}
