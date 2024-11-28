use std::fmt::{Debug, Formatter};

#[derive(Hash, Eq, Copy, Clone)]
pub struct Coord2 {
    pub x: i32,
    pub y: i32,
}

impl Coord2 {
    pub fn new(x: i32, y: i32) -> Coord2 {
        Coord2 { x, y }
    }

    pub fn add(&self, point: &Coord2) -> Coord2 {
        Coord2 {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

impl PartialEq for Coord2 {
    fn eq(&self, other: &Coord2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Debug for Coord2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord2[{}, {}]", self.x, self.y)
    }
}