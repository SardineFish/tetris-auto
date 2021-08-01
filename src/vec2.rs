use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub i8, pub i8);
impl ops::Add for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}