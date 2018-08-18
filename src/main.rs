mod vecmath;

use std::fs::File;
use std::io::prelude::*;
use vecmath::Vec3;

fn main() {
    let mut ray_file = File::create("raycast.ppm").unwrap();
    let width = 200;
    let height = 100;
    let header = format!("P3\n{} {}\n255\n", width, height);
    ray_file.write_all(header.as_bytes()).unwrap();
    for x in 0..height {
        for y in 0..width {
            let color = Vec3::from(
                (y as f32 / width as f32) * 255.9,
                (x as f32 / height as f32) * 255.9,
                0.2 * 255.9
            );
            ray_file.write_all(format!("{}\n", color).as_bytes()).unwrap();
        }
    }
}