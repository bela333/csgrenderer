use crate::{objects::Object, range_vec_union::RangeVecUnion};

#[derive(Clone)]
pub struct CSGVecUnion<O: Object> {
    objects: Vec<O>,
}

impl<O: Object> CSGVecUnion<O> {
    pub fn new(objects: Vec<O>) -> Self {
        Self { objects }
    }
}

impl<O: Object> Object for CSGVecUnion<O> {
    type Iter = RangeVecUnion<O::Iter>;

    fn trace(&self, origin: glam::Vec3, direction: glam::Vec3) -> Self::Iter {
        let is: Vec<O::Iter> = self
            .objects
            .iter()
            .map(|obj| obj.trace(origin, direction))
            .collect();
        RangeVecUnion::new(is)
    }
}
