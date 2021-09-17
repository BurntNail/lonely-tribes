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