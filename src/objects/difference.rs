use crate::{objects::Object, range_difference::RangeDifference};

#[derive(Clone)]
pub struct CSGDifference<O1, O2>
where
    O1: Object,
    O2: Object,
{
    obj1: O1,
    obj2: O2,
}

impl<O1, O2> CSGDifference<O1, O2>
where
    O1: Object,
    O2: Object,
{
    pub fn new(obj1: O1, obj2: O2) -> Self {
        Self { obj1, obj2 }
    }
}

impl<O1, O2> Object for CSGDifference<O1, O2>
where
    O1: Object,
    O2: Object,
{
    type Iter = RangeDifference<O1::Iter, O2::Iter>;

    fn trace(&self, origin: glam::Vec3, direction: glam::Vec3) -> Self::Iter {
        let i1 = self.obj1.trace(origin, direction);
        let i2 = self.obj2.trace(origin, direction);
        Self::Iter::new(i1, i2)
    }
}
