use glam::Vec3;

use crate::{objects::Object, range_intersect::RangeIntersect};

pub struct CSGClipplane<O: Object> {
    obj: O,
    normal: Vec3,
    d: f32,
}

impl<O: Object> Object for CSGClipplane<O> {
    type Iter = RangeIntersect<O::Iter, std::vec::IntoIter<f32>>;

    // TODO: Reimplement with own iterator
    fn trace(&self, origin: Vec3, direction: Vec3) -> Self::Iter {
        // <self.normal, origin + direction * t> = self.d
        // <self.normal, origin> + <self.normal, direction * t> = self.d
        // <self.normal, origin> + t * <self.normal, direction> = self.d
        // t * <self.normal, direction> = self.d - <self.normal, origin>
        // t = ( self.d - <self.normal, origin> )  / <self.normal, direction>
        let nd = self.normal.dot(direction);
        if nd == 0.0 {
            return RangeIntersect::new(
                self.obj.trace(origin, direction),
                vec![-f32::INFINITY, f32::INFINITY].into_iter(),
            );
        }
        let threshold = (self.d - self.normal.dot(origin)) / self.normal.dot(direction);
        let far = if nd < 0.0 {
            f32::INFINITY
        } else {
            -f32::INFINITY
        };
        RangeIntersect::new(
            self.obj.trace(origin, direction),
            vec![threshold.min(far), threshold.max(far)].into_iter(),
        )
    }
}

impl<O: Object> CSGClipplane<O> {
    pub fn new(obj: O, normal: Vec3, d: f32) -> Self {
        Self {
            obj,
            normal: normal.normalize(),
            d,
        }
    }
}
