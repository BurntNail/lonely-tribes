#[derive(Copy, Clone, Debug)]
pub enum Either<T1, T2> {
    One(T1),
    Two(T2),
}
impl<T1, T2> Either<T1, T2> {
    pub fn is_one(&self) -> bool {
        matches!(self, Self::One(_))
    }
    pub fn is_two(&self) -> bool {
        matches!(self, Self::Two(_))
    }
}

impl<T1: PartialEq, T2: PartialEq> PartialEq for Either<T1, T2> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl<T1: Eq + PartialEq, T2: Eq + PartialEq> Eq for Either<T1, T2> {}
