#![allow(dead_code)]
use image::{imageops, GenericImageView, ImageBuffer, Rgb, RgbImage};
use rayon::prelude::*;
use vek::{Quaternion, Sphere, Transform, Vec2, Vec3};
mod camera;
mod drawable;
mod ray;
mod sphere;

use camera::Camera;
use drawable::{Drawable, Intersection};
use ray::Ray;

fn main() {
    let width = 800;
    let height = 600;
    let output_size = Vec2::new(800, 600);

    let s1: Sphere<f32, f32> = Sphere::new(Vec3::new(30.0, 0.0, 0.0), 10.0);
    println!("Hello {:?}", s1);

    let camera_transform: Transform<f32, f32, f32> = Transform {
        position: Vec3::zero(),
        orientation: Quaternion::zero(),
        scale: Vec3::one(),
    };
    let aspect_ratio = width as f32 / height as f32;
    let camera = Camera::new(camera_transform, 90.0, aspect_ratio, 0.1, 1000.0);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img = RgbImage::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let clip_x = x as f32 / width as f32 * 2.0 - 1.0;
        let clip_y = y as f32 / height as f32 * 2.0 - 1.0;
        let ray = camera.generate_ray(Vec2::new(clip_x, clip_y));
        match s1.intersect(&ray) {
            Some(_) => *pixel = Rgb([200, 0, 100]),
            None => *pixel = Rgb([(0.3 * x as f32) as u8, 0, (0.3 * y as f32) as u8]),
        }
    }

    img.save("test.png").unwrap();

    (0..output_size.x + 1).into_par_iter().for_each(|x| {
        for y in 0..output_size.y {
            let ray =
                camera.generate_ray(Vec2::new(x as f32 / width as f32, y as f32 / height as f32));
        }
    });
}
