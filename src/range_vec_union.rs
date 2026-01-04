use std::iter::Peekable;

struct UnionStateMachine<I: Iterator> {
    pub iterator: Peekable<I>,
    pub inside: bool,
}
pub struct RangeVecUnion<I>
where
    I: Iterator<Item = f32>,
{
    machines: Vec<UnionStateMachine<I>>,
    count: usize,
}

impl<I> Iterator for RangeVecUnion<I>
where
    I: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let count_before = self.count;
        let (sm, _) = self
            .machines
            .iter_mut()
            .filter_map(|sm| sm.iterator.peek().map(ToOwned::to_owned).map(|v| (sm, v)))
            .min_by(|(_, va), (_, vb)| va.partial_cmp(vb).unwrap_or(std::cmp::Ordering::Less))?;
        let v = sm.iterator.next().unwrap();
        if sm.inside {
            self.count -= 1;
        } else {
            self.count += 1;
        }
        sm.inside = !sm.inside;
        if count_before == 0 || self.count == 0 {
            Some(v)
        } else {
            self.next()
        }
    }
}

impl<I> RangeVecUnion<I>
where
    I: Iterator<Item = f32>,
{
    pub fn new(objects: Vec<I>) -> Self {
        Self {
            machines: objects
                .into_iter()
                .map(|v| UnionStateMachine {
                    iterator: v.peekable(),
                    inside: false,
                })
                .collect(),
            count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::range_vec_union::RangeVecUnion;

    #[test]
    fn union_null() {
        let obj1 = vec![];
        let obj2 = vec![];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![]);
    }

    #[test]
    fn union_single() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![0.0, 1.0]);

        let obj1 = vec![];
        let obj2 = vec![0.0, 1.0];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![0.0, 1.0]);
    }

    #[test]
    fn union_intersect_once() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![0.5, 2.0];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![0.0, 2.0]);
    }

    #[test]
    fn union_contain() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![0.5, 0.7];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![0.0, 1.0]);
    }

    #[test]
    fn union_multiple() {
        let obj1 = vec![0.0, 1.0, 2.0, 3.0];
        let obj2 = vec![0.5, 2.5];
        let union: Vec<f32> =
            RangeVecUnion::new(vec![obj1.into_iter(), obj2.into_iter()]).collect();
        assert_eq!(union, vec![0.0, 3.0]);
    }
}
