use image;
use crate::vec3::Vec3;

// (0; 0) is top-left corner
// x -> right
// y -> bottom

pub fn run() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const PATH: &str = "out/out.png";

    let mut buffer = vec![];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (x * 255 / WIDTH) as u8;
            let g = (y * 255 / HEIGHT) as u8;
            let b = (WIDTH - x) as f32 / WIDTH as f32 * (HEIGHT - y) as f32 / HEIGHT as f32;
            let b = (b * 255.) as u8;
            // let b = 0;
            buffer.extend([r, g, b]);
        }
    }

    image::save_buffer(
        PATH,
        &buffer,
        WIDTH,
        HEIGHT,
        image::ColorType::Rgb8,
    ).unwrap();
}