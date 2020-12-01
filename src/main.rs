#![allow(dead_code)]
use indicatif::{ProgressBar, ProgressStyle};
use vek::{Quaternion, Sphere, Transform, Vec3};
mod camera;
mod drawable;
mod ray;
mod scene;
mod sphere;

use camera::Camera;
use scene::Scene;

fn main() {
    let width = 1280;
    let height = 720;

    let camera_transform: Transform<f32, f32, f32> = Transform {
        position: Vec3::zero(),
        orientation: Quaternion::zero(),
        scale: Vec3::one(),
    };
    let aspect_ratio = width as f32 / height as f32;
    let camera = Camera::new(camera_transform, 90.0, aspect_ratio, 0.1, 1000.0);

    let mut scene = Scene::new(camera);

    let s1: Sphere<f32, f32> = Sphere::new(Vec3::new(30.0, 0.0, 0.0), 10.0);
    scene.spheres.push(s1);

    let bar = ProgressBar::new(1);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"),
    );

    let img = scene.render(width, height);
    img.save("test.png").unwrap();

    bar.finish();
}
