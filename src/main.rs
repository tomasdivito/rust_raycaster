mod vecmath;
mod raymath;

use std::fs::File;
use std::io::prelude::*;
use vecmath::Vec3;
use raymath::Ray;

fn main() {
    let mut ray_file = File::create("raycast.ppm").unwrap();
    let width = 200;
    let height = 100;
    let header = String::from(format!("P3\n{} {}\n255\n", width, height));
    ray_file.write_all(header.as_bytes()).unwrap();
    for x in 0..height {
        for y in 0..width {
            let color = Vec3::from(
                (y as f32 / width as f32) * 255.9,
                (x as f32 / height as f32) * 255.9,
                0.2 * 255.9
            );
            ray_file.write_all(String::from(format!("{}\n", color)).as_bytes()).unwrap();
        }
    }
}