use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl ops::Mul<&f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl ops::Mul<&u32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &u32) -> Vec3 {
        Vec3::new(
            self.x * *other as f32,
            self.y * *other as f32,
            self.z * *other as f32,
        )
    }
}

impl ops::Mul<&Vec3> for &f32 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl ops::Mul<&Vec3> for &u32 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            *self as f32 * other.x,
            *self as f32 * other.y,
            *self as f32 * other.z,
        )
    }
}

impl ops::Div<&f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: &f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl ops::Div<&u32> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: &u32) -> Vec3 {
        Vec3::new(
            self.x / *other as f32,
            self.y / *other as f32,
            self.z / *other as f32,
        )
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        let prod = self * other;
        prod.x + prod.y + prod.z
    }

    pub fn norm(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            &self.y * &other.z - &self.z * &other.y,
            &self.z * &other.x - &self.x * &other.z,
            &self.x * &other.y - &self.y * &other.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / &self.norm()
    }
}
