use parse_display::FromStr;

#[derive(Copy, Clone, Debug, Eq, Hash, FromStr, PartialOrd, Ord)]
// Parses [1, 2] or (1, 2) or {1, 2}
#[from_str(regex = r"[\[\(\{](?<x>-?\d+),\s*(?<y>-?\d+)[\]\)\}]")]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl<T> PartialEq<T> for Vec2D where T: Into<Vec2D> + Copy {
    fn eq(&self, other: &T) -> bool {
        let other: Vec2D = (*other).into();
        self.x == other.x && self.y == other.y
    }
}

impl Vec2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn xy_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    pub fn xaxis(x: i64) -> Self {
        Self { x, y: 0 }
    }

    pub fn yaxis(y: i64) -> Self {
        Self { x: 0, y }
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn flipped(&self) -> Self {
        Self::new(self.y, self.x)
    }

    pub fn rotated_right(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn rotated_left(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn manhattan_dist(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn both_coordinates_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn inside_box(&self, lower: impl Into<Vec2D>, upper: impl Into<Vec2D>) -> bool {
        let (lower, upper) = (lower.into(), upper.into());
        self.x >= lower.x && self.y >= lower.y && self.x <= upper.x && self.y <= upper.y
    }

    pub fn diagonal_neighbors(&self) -> impl Iterator<Item = Vec2D> + '_ {
        [
            Vec2D { x: 1, y: 1 },
            Vec2D { x: -1, y: 1 },
            Vec2D { x: 1, y: -1 },
            Vec2D { x: -1, y: -1 },
        ]
        .iter()
        .map(move |&dir| *self + dir)
    }

    pub fn orthogonal_neighbors(&self) -> impl Iterator<Item = Vec2D> + '_ {
        [
            Vec2D { x: 1, y: 0 },
            Vec2D { x: -1, y: 0 },
            Vec2D { x: 0, y: 1 },
            Vec2D { x: 0, y: -1 },
        ]
        .iter()
        .map(move |&dir| *self + dir)
    }

    pub fn all_neighbors(&self) -> impl Iterator<Item = Vec2D> + '_ {
        self.diagonal_neighbors().chain(self.orthogonal_neighbors())
    }
}

/// This would of course be horrible in productions, but it make the Advent of Code solutions much
/// more readable. We assume that we never need any positions whose coordinates are out of i64
/// bounds.
impl<T, U> From<(T, U)> for Vec2D
where
    T: num::ToPrimitive,
    U: num::ToPrimitive,
{
    fn from((x, y): (T, U)) -> Self {
        Self::new(x.to_i64().unwrap(), y.to_i64().unwrap())
    }
}

impl<T, U> From<&(T, U)> for Vec2D
where
    T: num::ToPrimitive + Copy,
    U: num::ToPrimitive + Copy,
{
    fn from((x, y): &(T, U)) -> Self {
        Self::from((*x, *y))
    }
}

/// Again, horrible in production, but nice for AoC.
/// This way we don't need to manually convert before multiplying.
impl<T> std::ops::Mul<T> for Vec2D
where
    T: num::ToPrimitive + num::Integer,
{
    type Output = Vec2D;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2D::new(
            self.x * rhs.to_i64().unwrap(),
            self.y * rhs.to_i64().unwrap(),
        )
    }
}

impl <T> std::ops::MulAssign<T> for Vec2D
where
    T: num::ToPrimitive + num::Integer,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

// Doing this generically violates Rust's orphan rules.
// https://users.rust-lang.org/t/left-multiplication-for-custom-template-type/70882
macro_rules! impl_left_mul {
    ($($t:ty),*) => {
        $(
            impl std::ops::Mul<Vec2D> for $t {
                type Output = Vec2D;
            
                fn mul(self, rhs: Vec2D) -> Self::Output {
                    rhs * self
                }
            }
        )*
    };
}
impl_left_mul!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl<T> std::ops::Add<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> std::ops::AddAssign<T> for Vec2D where T: Into<Vec2D> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> std::ops::Sub<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> std::ops::SubAssign<T> for Vec2D where T: Into<Vec2D> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn arithmetics() {
        let mut vec = Vec2D::new(0, 0) + (-1, 1);
        assert_eq!(vec, (-1, 1));
        assert_eq!(vec.xy_tuple(), (-1, 1));

        vec += (1, 2);
        assert_eq!(vec, (0, 3));

        vec -= (1, 2);
        assert_eq!(vec, (-1, 1));

        vec = vec - vec;
        assert_eq!(vec, Vec2D::zero());

        vec = Vec2D::new(1, 2) * 2;
        assert_eq!(vec, (2, 4));
        vec *= -2;
        assert_eq!(vec, (-4, -8));

        vec = 2 * vec * 2;
        assert_eq!(vec, (-16, -32));
    }

    #[test]
    fn from_str() {
        assert_eq!("(1,    2)".parse(), Ok(Vec2D::new(1, 2)));
        assert_eq!("(1, 2)".parse(), Ok(Vec2D::new(1, 2)));
        assert_eq!("(1,2)".parse(), Ok(Vec2D::new(1, 2)));
        assert_eq!("(-1, 2)".parse(), Ok(Vec2D::new(-1, 2)));
        assert_eq!("(1, -2)".parse(), Ok(Vec2D::new(1, -2)));
        assert_eq!("[1, 2]".parse(), Ok(Vec2D::new(1, 2)));
        assert_eq!("{1, 2}".parse(), Ok(Vec2D::new(1, 2)));
    }

    #[test]
    fn rotate_and_flip() {
        assert_eq!(Vec2D::new(1, 2).flipped(), Vec2D::new(2, 1));
        assert_eq!(Vec2D::new(1, 2).rotated_right(), Vec2D::new(-2, 1));
        assert_eq!(Vec2D::new(1, 2).rotated_left(), Vec2D::new(2, -1));
    }

    #[test]
    fn manhattan_dist() {
        assert_eq!(Vec2D::new(1, 2).manhattan_dist(), 3);
        assert_eq!(Vec2D::new(-1, 2).manhattan_dist(), 3);
        assert_eq!(Vec2D::new(1, -2).manhattan_dist(), 3);
    }

    #[test] 
    fn inside_box() {
        assert!(Vec2D::new(1, 2).inside_box((0, 0), (2, 3)));
        assert!(Vec2D::new(1, 2).inside_box((1, 2), (1, 2)));
        assert!(!Vec2D::new(1, 2).inside_box((0, 0), (1, 1)));
        assert!(!Vec2D::new(1, 2).inside_box((2, 3), (3, 4)));

        assert!(Vec2D::new(-1, -2).inside_box((-2, -3), (0, 0)));
        assert!(Vec2D::new(-1, -2).inside_box((-1, -2), (-1, -2)));
        assert!(!Vec2D::new(-1, -2).inside_box((0, 0), (1, 1)));
        assert!(!Vec2D::new(-1, -2).inside_box((-3, -4), (-2, -3)));

        assert!(Vec2D::new(-1, 2).inside_box((-2, 0), (0, 3)));
        assert!(Vec2D::new(-1, 2).inside_box((-1, 2), (-1, 2)));
        assert!(!Vec2D::new(-1, 2).inside_box((0, 0), (1, 1)));
        assert!(!Vec2D::new(-1, 2).inside_box((-3, 3), (-2, 4)));
    }

    #[test]
    fn from() {
        assert_eq!(Vec2D::from((1, 2)), Vec2D::new(1, 2));
        assert_eq!(Vec2D::from((1_u8, 2_u8)), Vec2D::new(1, 2));
        assert_eq!(Vec2D::from((1_i8, 2_i8)), Vec2D::new(1, 2));
        assert_eq!(Vec2D::from((1_usize, 2_usize)), Vec2D::new(1, 2));
    }

    #[test]
    fn neighbors() {
        assert_eq!(
            HashSet::from([
                Vec2D::new(2, 2),
                Vec2D::new(0, 2),
                Vec2D::new(1, 3),
                Vec2D::new(1, 1)
            ]),
            Vec2D::new(1, 2).orthogonal_neighbors().collect(),
        );
        assert_eq!(
            HashSet::from([
                Vec2D::new(2, 3),
                Vec2D::new(0, 3),
                Vec2D::new(2, 1),
                Vec2D::new(0, 1)
            ]),
            Vec2D::new(1, 2).diagonal_neighbors().collect(),
        );
        assert_eq!(
            HashSet::from([
                Vec2D::new(2, 2),
                Vec2D::new(0, 2),
                Vec2D::new(1, 3),
                Vec2D::new(1, 1),
                Vec2D::new(2, 3),
                Vec2D::new(0, 3),
                Vec2D::new(2, 1),
                Vec2D::new(0, 1)
            ]),
            Vec2D::new(1, 2).all_neighbors().collect(),
        );
    }
}
