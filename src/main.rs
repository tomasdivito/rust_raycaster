mod raytracer;

use std::fs::File;
use std::io::prelude::*;

use raytracer::vec::Vec3;
use raytracer::ray::Ray;

fn main() {
    let mut ray_file = File::create("raycast.ppm").unwrap();
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3::from(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from(4.0, 0.0, 0.0);
    let vertical = Vec3::from(0.0, 2.0, 0.0);
    let origin = Vec3::from(0.0, 0.0, 0.0);

    let header = format!("P3\n{} {}\n255\n", width, height);
    ray_file.write_all(header.as_bytes()).unwrap();

    for y in (0..height).rev() {
        for x in 0..width {
            let u: f32 = width as f32 / x as f32;
            let v: f32 = height as f32 / y as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&ray);

            let ir = (col.x * 255.99) as i64;
            let ig = (col.y * 255.99) as i64;
            let ib = (col.z * 255.99) as i64;
            ray_file
                .write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::from(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::from(0.5, 0.7, 1.0) * t
}
