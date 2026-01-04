use glam::Vec3;

pub mod intersect;
pub mod sphere;
pub mod transform;
pub mod union;

pub trait Object
where
    Self::Iter: Iterator<Item = f32>,
{
    type Iter;

    fn trace(&self, origin: Vec3, direction: Vec3) -> Self::Iter;
}
