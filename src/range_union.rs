use std::iter::Peekable;

struct UnionStateMachine<I: Iterator> {
    pub iterator: Peekable<I>,
    pub inside: bool,
}

pub struct RangeUnion<I1, I2>
where
    I1: Iterator<Item = f32>,
    I2: Iterator<Item = f32>,
{
    obj1: UnionStateMachine<I1>,
    obj2: UnionStateMachine<I2>,
}

impl<I1, I2> RangeUnion<I1, I2>
where
    I1: Iterator<Item = f32>,
    I2: Iterator<Item = f32>,
{
    pub fn new(obj1: I1, obj2: I2) -> Self {
        Self {
            obj1: UnionStateMachine {
                iterator: obj1.peekable(),
                inside: false,
            },
            obj2: UnionStateMachine {
                iterator: obj2.peekable(),
                inside: false,
            },
        }
    }
}

impl<I1, I2> Iterator for RangeUnion<I1, I2>
where
    I1: Iterator<Item = f32>,
    I2: Iterator<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match (self.obj1.iterator.peek(), self.obj2.iterator.peek()) {
            (None, None) => None,
            (None, Some(_)) => self.obj2.iterator.next(),
            (Some(_), None) => self.obj1.iterator.next(),
            (Some(a), Some(b)) => {
                if a < b {
                    let this = &mut self.obj1;
                    let other = &mut self.obj2;
                    let v = this.iterator.next();
                    this.inside = !this.inside;
                    if !other.inside { v } else { self.next() }
                } else {
                    let this = &mut self.obj2;
                    let other = &mut self.obj1;
                    let v = this.iterator.next();
                    this.inside = !this.inside;
                    if !other.inside { v } else { self.next() }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::range_union::RangeUnion;

    #[test]
    fn union_null() {
        let obj1 = vec![];
        let obj2 = vec![];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![]);
    }

    #[test]
    fn union_single() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![0.0, 1.0]);

        let obj1 = vec![];
        let obj2 = vec![0.0, 1.0];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![0.0, 1.0]);
    }

    #[test]
    fn union_intersect_once() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![0.5, 2.0];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![0.0, 2.0]);
    }

    #[test]
    fn union_contain() {
        let obj1 = vec![0.0, 1.0];
        let obj2 = vec![0.5, 0.7];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![0.0, 1.0]);
    }

    #[test]
    fn union_multiple() {
        let obj1 = vec![0.0, 1.0, 2.0, 3.0];
        let obj2 = vec![0.5, 2.5];
        let union: Vec<f32> = RangeUnion::new(obj1.into_iter(), obj2.into_iter()).collect();
        assert_eq!(union, vec![0.0, 3.0]);
    }
}
