use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
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

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl ops::Mul<i64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: i64) -> Vec3 {
        self * other as f64
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Mul<&Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl ops::Div<i64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: i64) -> Vec3 {
        self / other as f64
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        let prod = self * other;
        prod.x + prod.y + prod.z
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec_1() {
        let a: Vec3 = Vec3::new(0., 0., 0.);
        let b: Vec3 = Vec3::new(0., 0., 0.);
        let res: Vec3 = Vec3::new(0., 0., 0.);

        assert_eq!(&a + &b, res);
    }

    #[test]
    fn test_add_vec_2() {
        let a: Vec3 = Vec3::new(7., -2., -6.4);
        let b: Vec3 = Vec3::new(2.9, -0.1, 0.9);
        let res: Vec3 = Vec3::new(9.9, -2.1, -5.5);

        assert_eq!(&a + &b, res);
    }

    #[test]
    fn test_sub_vec_1() {
        let a: Vec3 = Vec3::new(0., 0., 0.);
        let b: Vec3 = Vec3::new(0., 0., 0.);
        let res: Vec3 = Vec3::new(0., 0., 0.);

        assert_eq!(&a - &b, res);
    }

    #[test]
    fn test_sub_vec_2() {
        let a: Vec3 = Vec3::new(7., -2., -6.4);
        let b: Vec3 = Vec3::new(2.9, -0.1, 0.9);
        let sub: Vec3 = &a - &b;
        let res: Vec3 = Vec3::new(4.1, -1.9, -7.3);

        let epsilon: f64 = 0.000001;

        assert!((sub.x - res.x).abs() < epsilon);
        assert!((sub.y - res.y).abs() < epsilon);
        assert!((sub.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_mul_vec_1() {
        let a: Vec3 = Vec3::new(1., 1., 1.);
        let b: Vec3 = Vec3::new(1., 1., 1.);
        let res: Vec3 = Vec3::new(1., 1., 1.);

        assert_eq!(&a * &b, res);
    }

    #[test]
    fn test_mul_vec_2() {
        let a: Vec3 = Vec3::new(1., 1., 1.);
        let b: Vec3 = Vec3::new(-3., 4., -2.5);
        let res: Vec3 = Vec3::new(-3., 4., -2.5);

        assert_eq!(&a * &b, res);
    }

    #[test]
    fn test_mul_vec_3() {
        let a: Vec3 = Vec3::new(2.9, 0., -3.5);
        let b: Vec3 = Vec3::new(-3., 4., -2.);
        let mul: Vec3 = &a * &b;
        let res: Vec3 = Vec3::new(-8.7, 0., 7.);

        let epsilon: f64 = 0.000001;

        assert!((mul.x - res.x).abs() < epsilon);
        assert!((mul.y - res.y).abs() < epsilon);
        assert!((mul.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_div_vec_1() {
        let a: Vec3 = Vec3::new(1., 1., 1.);
        let b: Vec3 = Vec3::new(1., 1., 1.);
        let res: Vec3 = Vec3::new(1., 1., 1.);

        assert_eq!(&a / &b, res);
    }

    #[test]
    fn test_div_vec_2() {
        let a: Vec3 = Vec3::new(-3., 4., -2.5);
        let b: Vec3 = Vec3::new(1., 1., 1.);
        let res: Vec3 = Vec3::new(-3., 4., -2.5);

        assert_eq!(&a / &b, res);
    }

    #[test]
    fn test_div_vec_3() {
        let a: Vec3 = Vec3::new(2.9, 0., 4.);
        let b: Vec3 = Vec3::new(-2.9, 4., -2.);
        let div: Vec3 = &a / &b;
        let res: Vec3 = Vec3::new(-1., 0., -2.);

        let epsilon: f64 = 0.000001;

        assert!((div.x - res.x).abs() < epsilon);
        assert!((div.y - res.y).abs() < epsilon);
        assert!((div.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_mul_scalar_1() {
        let a: Vec3 = Vec3::new(1., 1., 1.);
        let b: f64 = 1.;
        let res: Vec3 = Vec3::new(1., 1., 1.);

        assert_eq!(&a * b, res);
    }

    #[test]
    fn test_mul_scalar_2() {
        let a: f64 = 1.;
        let b: Vec3 = Vec3::new(-3., 4., -2.5);
        let res: Vec3 = Vec3::new(-3., 4., -2.5);

        assert_eq!(a * &b, res);
    }

    #[test]
    fn test_mul_scalar_3() {
        let a: Vec3 = Vec3::new(2.9, 0., -3.5);
        let b: f64 = 3.2;
        let mul: Vec3 = &a * b;
        let res: Vec3 = Vec3::new(9.28, 0., -11.2);

        let epsilon: f64 = 0.000001;

        assert!((mul.x - res.x).abs() < epsilon);
        assert!((mul.y - res.y).abs() < epsilon);
        assert!((mul.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_mul_scalar_4() {
        let a: Vec3 = Vec3::new(2.9, 0., -3.5);
        let b: i64 = 4;
        let mul: Vec3 = &a * b;
        let res: Vec3 = Vec3::new(11.6, 0., -14.0);

        let epsilon: f64 = 0.000001;

        assert!((mul.x - res.x).abs() < epsilon);
        assert!((mul.y - res.y).abs() < epsilon);
        assert!((mul.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_div_scalar_1() {
        let a: Vec3 = Vec3::new(1., 1., 1.);
        let b: f64 = 1.;
        let res: Vec3 = Vec3::new(1., 1., 1.);

        assert_eq!(&a / b, res);
    }

    #[test]
    fn test_div_scalar_2() {
        let a: Vec3 = Vec3::new(2.9, 0., -3.5);
        let b: f64 = 3.2;
        let mul: Vec3 = &a / b;
        let res: Vec3 = Vec3::new(0.90625, 0., -1.09375);

        let epsilon: f64 = 0.000001;

        assert!((mul.x - res.x).abs() < epsilon);
        assert!((mul.y - res.y).abs() < epsilon);
        assert!((mul.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_div_scalar_3() {
        let a: Vec3 = Vec3::new(2.9, 0., -3.5);
        let b: i64 = 4;
        let mul: Vec3 = &a / b;
        let res: Vec3 = Vec3::new(0.725, 0., -0.875);

        let epsilon: f64 = 0.000001;

        assert!((mul.x - res.x).abs() < epsilon);
        assert!((mul.y - res.y).abs() < epsilon);
        assert!((mul.z - res.z).abs() < epsilon);
    }

    #[test]
    fn test_dot_1() {
        let a: Vec3 = Vec3::new(1., 2., 3.);
        let b: Vec3 = Vec3::new(0., 0., 0.);
        let res: f64 = 0.;

        assert_eq!(a.dot(&b), res);
    }

    #[test]
    fn test_dot_2() {
        let a: Vec3 = Vec3::new(1., 2., 3.);
        let b: Vec3 = Vec3::new(-2., 1., 4.);
        let res: f64 = 12.;

        assert_eq!(a.dot(&b), res);
    }

    #[test]
    fn test_norm_1() {
        let a: Vec3 = Vec3::new(0.,0.,0.);
        let res: f64 = 0.;

        assert_eq!(a.norm(), res);
    }

    #[test]
    fn test_norm_2() {
        let a: Vec3 = Vec3::new(1.,1.,1.);
        let res: f64 = 1.73205;
        let epsilon: f64 = 0.000001;

        assert!((a.norm() - res).abs() < epsilon);
    }

    #[test]
    fn test_norm_3() {
        let a: Vec3 = Vec3::new(1.,2.5,-1.2);
        let res: f64 = 2.94788;
        let epsilon: f64 = 0.000001;

        println!("{}", a.norm());

        assert!((a.norm() - res).abs() < epsilon);
    }
}
