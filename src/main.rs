mod aymeric;
mod color;
mod ray;
mod vec3;

fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const PATH: &str = "out/out.png";
    aymeric::run(WIDTH, HEIGHT, PATH);
    // aymeric::run(WIDTH, HEIGHT);
}
