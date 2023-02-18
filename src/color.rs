use crate::vec3;

pub fn write_color(pixel_color: vec3::Vec3) {
    println!(
        "{} {} {}\n",
        255.999 * pixel_color.x(),
        255.999 * pixel_color.y(),
        255.999 * pixel_color.z()
    );
}
