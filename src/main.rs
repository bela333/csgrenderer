use std::f32;

use glam::{Affine3A, Mat3, Vec3, vec3};
use image::RgbImage;

use crate::{
    objects::{
        Object, cylinder::CSGCylinder, difference::CSGDifference, sphere::CSGSphere,
        transform::CSGTransform, vec_union::CSGVecUnion,
    },
    range_intersect::RangeIntersect,
};

pub mod objects;
pub mod range_difference;
pub mod range_intersect;
pub mod range_union;
pub mod range_vec_union;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn main() {
    let o = {
        let sphere = CSGSphere::new(Vec3::ZERO, 1.0);
        let planar_holes: CSGVecUnion<_> = CSGVecUnion::new(
            (0..5)
                .map(|i| {
                    let transform = Affine3A::from_rotation_x(-f32::consts::PI / 4.0 * (i as f32));
                    let cylinder = CSGCylinder::new(0.2, 1.0);
                    CSGTransform::new(cylinder, transform)
                })
                .collect(),
        );

        let holes = CSGVecUnion::new(
            (0..8)
                .map(|i| {
                    let planar_holes = planar_holes.clone();
                    let transform =
                        Affine3A::from_rotation_y(f32::consts::PI * 2.0 / 8.0 * (i as f32));

                    CSGTransform::new(planar_holes, transform)
                })
                .collect(),
        );
        CSGDifference::new(sphere, holes)
    };

    let light = vec3(2.0, 2.0, -2.0) * 100.0;
    for frame in 0..180 {
        let camera = Affine3A::look_at_lh(
            Mat3::from_rotation_y(((frame * 2) as f32).to_radians()) * vec3(3.0, 2.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )
        .inverse();

        let camera_origin: Vec3 = camera.translation.into();
        let mut buf = vec![[None; WIDTH as usize]; HEIGHT as usize];

        for (y, row) in buf.iter_mut().enumerate() {
            for (x, p) in row.iter_mut().enumerate() {
                let x = x as f32 / WIDTH as f32;
                let y = y as f32 / HEIGHT as f32;
                let x = (x - 0.5) * 2.0 * ASPECT_RATIO;
                let y = ((1.0 - y) - 0.5) * 2.0;
                let direction = camera.transform_vector3(Vec3::new(x, y, 2.0)).normalize();
                let i = o.trace(camera_origin, direction);
                let mut i = RangeIntersect::new(i, vec![0.0, f32::INFINITY].into_iter());
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
            let color = Vec3::ONE * (diffuse + 0.01).powf(1.0 / 2.2);
            *p = image::Rgb([
                (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                (color.z.clamp(0.0, 1.0) * 255.0) as u8,
            ])
        }
        img.save(format!("frames/frame{frame:04}.png")).unwrap();
    }
}
