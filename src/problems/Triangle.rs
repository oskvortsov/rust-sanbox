use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T
}

impl<T: PartialOrd + Default + Add<Output = T> + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if !sides.iter().all(|it| it > &T::default()) {
            return None
        }

        let mut sides = sides;
        sides.sort_unstable_by(|a, b| { a.partial_cmp(b).unwrap()} );

        let [a,b,c] = sides;
        if a + b > c {
            return Some(Self { a, b, c })
        }

        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
