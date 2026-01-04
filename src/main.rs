use glam::{Affine3A, Vec3, vec3};
use image::RgbImage;

use crate::{
    csg_intersect::CSGIntersect,
    objects::{Object, intersect::Intersect, sphere::Sphere, transform::Transform},
};

pub mod csg_intersect;
pub mod csg_union;
pub mod objects;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn main() {
    let s1 = Sphere::new(glam::Vec3::new(0.0, 0.0, 0.0), 1.0);
    let s2 = Sphere::new(glam::Vec3::new(0.0, 1.0, 0.0), 1.0);
    let o = Intersect::new(s1, s2);
    let o = Transform::new(
        o,
        Affine3A::from_rotation_x(15.0f32.to_radians())
            * Affine3A::from_rotation_z(-15.0f32.to_radians())
            * Affine3A::from_translation(vec3(0.0, -0.5, 0.0)),
    );

    let camera = Affine3A::look_at_lh(
        vec3(0.0, 0.0, -1.2),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    )
    .inverse();
    let light = vec3(2.0, 2.0, -2.0) * 100.0;

    let camera_origin: Vec3 = camera.translation.into();
    let mut buf = vec![[None; WIDTH as usize]; HEIGHT as usize];

    for (y, row) in buf.iter_mut().enumerate() {
        for (x, p) in row.iter_mut().enumerate() {
            let x = x as f32 / WIDTH as f32;
            let y = y as f32 / HEIGHT as f32;
            let x = (x - 0.5) * 2.0 * ASPECT_RATIO;
            let y = ((1.0 - y) - 0.5) * 2.0;
            let direction = camera.transform_vector3(Vec3::new(x, y, 1.0)).normalize();
            let i = o.trace(camera_origin, direction);
            let mut i = CSGIntersect::new(i, vec![0.0, f32::INFINITY].into_iter());
            *p = i.next().map(|d| direction * d + camera_origin)
        }
    }
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    for (x, y, p) in img.enumerate_pixels_mut() {
        let v = buf[y as usize][x as usize];
        let v = match v {
            Some(v) => v,
            None => {
                *p = image::Rgb([0, 0, 0]);
                continue;
            }
        };
        let normal = {
            let vx = buf[y as usize]
                .get(x as usize + 1)
                .unwrap_or(&None)
                .unwrap_or(v);
            let vy = buf
                .get(y as usize + 1)
                .map(|v| v[x as usize])
                .unwrap_or(None)
                .unwrap_or(v);
            (v - vx).cross(v - vy).normalize()
        };
        let diffuse = (light - v).normalize().dot(normal).clamp(0.0, 1.0);
        let color = Vec3::ONE * (diffuse + 0.001).powf(1.0 / 2.2);
        *p = image::Rgb([
            (color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }
    img.save("output.png").unwrap();
}
