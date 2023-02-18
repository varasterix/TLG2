use crate::vec3;
use std::fmt;
use vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.origin, self.direction)
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, other: f64) -> Vec3 {
        &self.origin + &(other * &self.direction)
    }
}
