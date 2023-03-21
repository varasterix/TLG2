use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
    }

    fn set_face_normal(r: &Ray, outward_normal: &Vec3) {}
}

impl Hittable for HitRecord {
    fn set_face_normal(r: &Ray, outward_normal: &Vec3) {
        let front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        let neg_outward_normal = -1.0 * outward_normal; // HACK: I know, this is ugly
        let normal = match front_face {
            true => outward_normal,
            false => &neg_outward_normal,
        };
    }
}
