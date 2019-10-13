use alga::general::{Additive, ClosedAdd, ClosedSub, Identity};
use nalgebra::{RealField, Scalar};
use num::Integer;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::direction::Direction;
use crate::math::Rect;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq)]
pub struct Vec2<T: Scalar> {
    pub x: T,
    pub y: T
}

impl<T: Scalar> Vec2<T> {
    pub fn new() -> Vec2<T>
    where
        T: Identity<Additive>
    {
        Vec2 {
            x: nalgebra::zero(),
            y: nalgebra::zero()
        }
    }

    pub fn from_values(x: T, y: T) -> Vec2<T> { Vec2 { x, y } }

    pub fn len(&self) -> T
    where
        T: RealField
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn primary_direction(&self, to: Self) -> Option<Direction>
    where
        T: ClosedSub + PartialOrd + Identity<Additive>
    {
        let diff_abs =
            Self::from_values(difference_abs(self.x, to.x), difference_abs(self.y, to.y));

        // Lateral movement
        if diff_abs.x > diff_abs.y {
            match self.x.partial_cmp(&to.x) {
                Some(Ordering::Less) => Some(Direction::Right),
                Some(Ordering::Greater) => Some(Direction::Left),
                _ => None
            }
        }
        // Vertical movement
        else if diff_abs.y > diff_abs.x {
            match self.y.partial_cmp(&to.y) {
                Some(Ordering::Less) => Some(Direction::Down),
                Some(Ordering::Greater) => Some(Direction::Up),
                _ => None
            }
        }
        // Not discernable if it should be lateral or vertical
        else {
            None
        }
    }

    pub fn neighbours(&self, limits: Option<Rect<T>>) -> Vec<Self>
    where
        T: Integer
    {
        unimplemented!()
    }

    pub fn surrounding(&self, limits: Option<Rect<T>>) -> Vec<Self>
    where
        T: Integer
    {
        unimplemented!()
    }
}

// Begin mathematical operators -----------------------------------------------

// Addition
impl<T: Scalar + ClosedAdd> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { Vec2::from_values(self.x + rhs.x, self.y + rhs.y) }
}

impl<T: Scalar + ClosedAdd> Add<(T, T)> for Vec2<T> {
    type Output = Self;

    fn add(self, (x, y): (T, T)) -> Self { Vec2::from_values(self.x + x, self.y + y) }
}

impl<T: Scalar + ClosedAdd> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self { Vec2::from_values(self.x + rhs, self.y + rhs) }
}

impl<T: Scalar + AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Scalar + AddAssign> AddAssign<(T, T)> for Vec2<T> {
    fn add_assign(&mut self, (x, y): (T, T)) {
        self.x += x;
        self.y += y;
    }
}

// Subtraction
impl<T: Scalar + ClosedSub> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { Vec2::from_values(self.x - rhs.x, self.y - rhs.y) }
}

impl<T: Scalar + ClosedSub> Sub<(T, T)> for Vec2<T> {
    type Output = Self;

    fn sub(self, (x, y): (T, T)) -> Self { Vec2::from_values(self.x - x, self.y - y) }
}

impl<T: Scalar + ClosedSub> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self { Vec2::from_values(self.x - rhs, self.y - rhs) }
}

impl<T: Scalar + SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Scalar + SubAssign> SubAssign<(T, T)> for Vec2<T> {
    fn sub_assign(&mut self, (x, y): (T, T)) {
        self.x -= x;
        self.y -= y;
    }
}

// Scalar multiplication
impl<T: Scalar + Add<Output = T> + Mul<Output = T>> Mul for Vec2<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T { self.x * rhs.x + self.y * rhs.y }
}

impl<T: Scalar + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self { Vec2::from_values(self.x * rhs, self.y * rhs) }
}

impl<T: Scalar + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self { Vec2::from_values(self.x / rhs, self.y / rhs) }
}

// End of mathematical operators ----------------------------------------------

// Helper function to determine the absolute positive difference between two
// Values, which don't have to be signed.
fn difference_abs<T>(a: T, b: T) -> T
where
    T: ClosedSub + PartialOrd
{
    if a > b {
        a - b
    }
    else {
        b - a
    }
}
