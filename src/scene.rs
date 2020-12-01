use crate::{camera::Camera, drawable::Drawable};
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use vek::{Sphere, Vec2, Vec3};

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere<f32, f32>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Scene {
            camera,
            spheres: Vec::new(),
        }
    }

    pub fn render_single_core(&self, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = RgbImage::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = self.render_pixel(width, height, x, y);
        }

        img
    }

    pub fn render(&self, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = RgbImage::new(width, height);

        img.enumerate_pixels_mut()
            .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
            .par_iter_mut()
            .for_each(|(x, y, pixel)| {
                **pixel = self.render_pixel(width, height, *x, *y);
            });

        img
    }

    fn render_pixel(&self, width: u32, height: u32, x: u32, y: u32) -> Rgb<u8> {
        let clip_x = x as f32 / width as f32 * 2.0 - 1.0;
        let clip_y = 1.0 - y as f32 / height as f32 * 2.0;
        let mut ray = self.camera.generate_ray(Vec2::new(clip_x, clip_y));

        let mut color = Rgb([(0.3 * x as f32) as u8, 0, (0.3 * y as f32) as u8]);
        for sphere in self.spheres.iter() {
            match sphere.intersect(&ray) {
                Some(its) => {
                    let temp: Vec3<f32> = 0.5 * 255.0 * (its.normal + Vec3::one());
                    color = Rgb([temp.x as u8, temp.y as u8, temp.z as u8]);
                    ray.maxt = its.t;
                }
                None => {}
            }
        }

        color
    }
}
