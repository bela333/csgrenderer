use std::f32;

use glam::{Affine3A, Vec3, vec3};
use image::RgbImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    objects::{
        Object, clipplane::CSGClipplane, cylinder::CSGCylinder, difference::CSGDifference,
        sphere::CSGSphere, transform::CSGTransform, vec_union::CSGVecUnion,
    },
    range_intersect::RangeIntersect,
};

pub mod objects;
pub mod range_difference;
pub mod range_intersect;
pub mod range_union;
pub mod range_vec_union;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn main() {
    let light = vec3(2.0, 2.0, -2.0) * 100.0;
    let camera = Affine3A::look_at_lh(
        vec3(3.0, 3.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    )
    .inverse();

    let camera_origin: Vec3 = camera.translation.into();

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
        CSGClipplane::new(CSGDifference::new(sphere, holes), vec3(0.0, 1.0, 0.0), 0.0)
    };

    let buf: Vec<Vec<Option<Vec3>>> = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            (0..WIDTH)
                .into_par_iter()
                .map(|x| {
                    let x = x as f32 / WIDTH as f32;
                    let y = y as f32 / HEIGHT as f32;
                    let x = (x - 0.5) * 2.0 * ASPECT_RATIO;
                    let y = ((1.0 - y) - 0.5) * 2.0;
                    let direction = camera.transform_vector3(Vec3::new(x, y, 2.0)).normalize();
                    let i = o.trace(camera_origin, direction);
                    let mut i = RangeIntersect::new(i, vec![0.0, f32::INFINITY].into_iter());
                    i.next().map(|d| direction * d + camera_origin)
                })
                .collect()
        })
        .collect();
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        let v = buf[y as usize][x as usize];
        let v = match v {
            Some(v) => v,
            None => {
                *p = image::Rgb([0, 0, 0]);
                return;
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
    });
    img.save("output.png").unwrap();
}
