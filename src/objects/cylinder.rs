use glam::Vec3Swizzles;

use crate::objects::Object;

#[derive(Clone)]
pub struct CSGCylinder {
    radius_squared: f32,
    height: f32,
}

impl CSGCylinder {
    pub fn new(radius: f32, height: f32) -> Self {
        Self {
            radius_squared: radius * radius,
            height,
        }
    }
}

impl Object for CSGCylinder {
    type Iter = std::vec::IntoIter<f32>;

    fn trace(&self, origin: glam::Vec3, direction: glam::Vec3) -> Self::Iter {
        let flat_direction = direction.xz();
        let flat_direction_length = flat_direction.length();
        let flat_direction = direction.xz().normalize();
        let flat_origin = origin.xz();
        let uoc = (flat_origin).dot(flat_direction);
        let d = uoc * uoc - flat_origin.length_squared() + self.radius_squared;
        if d < 0.0 {
            return vec![].into_iter();
        }
        let d = d.sqrt();
        let r1 = (-uoc - d) / flat_direction_length;
        let r2 = (-uoc + d) / flat_direction_length;
        if direction.z == 0.0 {
            return vec![r1, r2].into_iter();
        }
        // ( origin + r * direction ).y = x
        //  origin.y + r * direction.y = x
        //  r * direction.y = x-origin.y
        //  r = ( x-origin.y ) / direction.y
        let r_base = -origin.y / direction.y;
        let r_top = (self.height - origin.y) / direction.y;
        let (r1, r2) = (r1.max(r_base.min(r_top)), r2.min(r_base.max(r_top)));
        if r1 > r2 {
            return vec![].into_iter();
        }
        vec![r1, r2].into_iter()
    }
}
