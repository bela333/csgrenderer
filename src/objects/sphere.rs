use glam::Vec3;

use crate::objects::Object;

#[derive(Clone)]
pub struct CSGSphere {
    origin: Vec3,
    radius_squared: f32,
}

impl CSGSphere {
    pub fn new(origin: Vec3, radius: f32) -> Self {
        Self {
            origin,
            radius_squared: radius * radius,
        }
    }
}

impl Object for CSGSphere {
    type Iter = std::vec::IntoIter<f32>;

    fn trace(&self, origin: Vec3, direction: Vec3) -> Self::Iter {
        let uoc = (origin - self.origin).dot(direction);
        let d = uoc * uoc - (origin - self.origin).length_squared() + self.radius_squared;
        if d < 0.0 {
            vec![].into_iter()
        } else {
            let d = d.sqrt();
            let r1 = -uoc - d;
            let r2 = -uoc + d;
            vec![r1, r2].into_iter()
        }
    }
}
