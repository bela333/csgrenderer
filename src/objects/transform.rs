use glam::Affine3A;

use crate::objects::Object;

#[derive(Clone)]
pub struct CSGTransform<O: Object> {
    obj: O,
    transformation: Affine3A,
}

impl<O: Object> CSGTransform<O> {
    pub fn new(obj: O, transformation: Affine3A) -> Self {
        Self {
            obj,
            transformation: transformation.inverse(),
        }
    }
}

impl<O: Object> Object for CSGTransform<O> {
    type Iter = O::Iter;

    fn trace(&self, origin: glam::Vec3, direction: glam::Vec3) -> Self::Iter {
        let origin = self.transformation.transform_point3(origin);
        let direction = self.transformation.transform_vector3(direction).normalize();
        self.obj.trace(origin, direction)
    }
}
