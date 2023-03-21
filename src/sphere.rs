use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = Ray::origin(r) - &self.center;
        let a = r.direction().norm().powi(2);
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.norm() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = &(&rec.p - &self.center) / self.radius;
        rec.normal = &(&rec.p - &self.center) / self.radius;
        Self::set_face_normal(r, &outward_normal);

        true
    }
}
