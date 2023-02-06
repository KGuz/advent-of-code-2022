#![allow(unused)]

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg},
    process::Output,
    str::FromStr,
};

use itertools::Itertools;

use crate::days::{captures, re};

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Point2d<T> {
    pub y: T,
    pub x: T,
}

impl<T: Display> Display for Point2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(y: {}, x: {})", self.y, self.x)
    }
}
impl<T: Debug> Debug for Point2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(y: {:?}, x: {:?})", self.y, self.x)
    }
}

// Operators

impl<T: Add<Output = T>> Add<Point2d<T>> for Point2d<T> {
    type Output = Self;
    fn add(self, rhs: Point2d<T>) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}
impl<T: Add<Output = T> + Copy> Add<T> for Point2d<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            y: self.y + rhs,
            x: self.x + rhs,
        }
    }
}
impl<T: Sub<Output = T>> Sub<Point2d<T>> for Point2d<T> {
    type Output = Self;
    fn sub(self, rhs: Point2d<T>) -> Self::Output {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}
impl<T: Sub<Output = T> + Copy> Sub<T> for Point2d<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            y: self.y - rhs,
            x: self.x - rhs,
        }
    }
}
impl<T: Mul<Output = T>> Mul<Point2d<T>> for Point2d<T> {
    type Output = Self;
    fn mul(self, rhs: Point2d<T>) -> Self::Output {
        Self {
            y: self.y * rhs.y,
            x: self.x * rhs.x,
        }
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Point2d<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }
}
impl<T: Div<Output = T>> Div<Point2d<T>> for Point2d<T> {
    type Output = Self;
    fn div(self, rhs: Point2d<T>) -> Self::Output {
        Self {
            y: self.y / rhs.y,
            x: self.x / rhs.x,
        }
    }
}
impl<T: Div<Output = T> + Copy> Div<T> for Point2d<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            y: self.y / rhs,
            x: self.x / rhs,
        }
    }
}
impl<T: AddAssign> AddAssign<Point2d<T>> for Point2d<T> {
    fn add_assign(&mut self, rhs: Point2d<T>) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}
impl<T: AddAssign + Copy> AddAssign<T> for Point2d<T> {
    fn add_assign(&mut self, rhs: T) {
        self.y += rhs;
        self.x += rhs;
    }
}
impl<T: SubAssign> SubAssign<Point2d<T>> for Point2d<T> {
    fn sub_assign(&mut self, rhs: Point2d<T>) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}
impl<T: SubAssign + Copy> SubAssign<T> for Point2d<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.y -= rhs;
        self.x -= rhs;
    }
}
impl<T: MulAssign> MulAssign<Point2d<T>> for Point2d<T> {
    fn mul_assign(&mut self, rhs: Point2d<T>) {
        self.y *= rhs.y;
        self.x *= rhs.x;
    }
}
impl<T: MulAssign + Copy> MulAssign<T> for Point2d<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.y *= rhs;
        self.x *= rhs;
    }
}
impl<T: DivAssign> DivAssign<Point2d<T>> for Point2d<T> {
    fn div_assign(&mut self, rhs: Point2d<T>) {
        self.y /= rhs.y;
        self.x /= rhs.x;
    }
}
impl<T: DivAssign + Copy> DivAssign<T> for Point2d<T> {
    fn div_assign(&mut self, rhs: T) {
        self.y /= rhs;
        self.x /= rhs;
    }
}

// Conversions

impl<T: FromStr> FromStr for Point2d<T> {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = re!(r"(-?\d+).(-?\d+)");
        let Some(caps) = re.captures(s) else { return Err(r"Pattern (-?\d+).(-?\d+). not recognised in input") };
        let Some(nums) = caps.iter().skip(1).map(|cap| cap.map(|cap| cap.as_str())).collect_tuple() else {
            return Err("Number of elements in string is incorrect")
        };
        let (Some(x), Some(y)) = nums else { return Err("Number of elements in string is incorrect") };
        let (Ok(x), Ok(y)) = (x.parse(), y.parse()) else { return Err("Could not parse one or more elements") };
        Ok(Self { y, x })
    }
}

impl<T> From<(T, T)> for Point2d<T> {
    fn from((y, x): (T, T)) -> Self {
        Point2d { y, x }
    }
}
impl<T> From<[T; 2]> for Point2d<T> {
    fn from([y, x]: [T; 2]) -> Self {
        Point2d { y, x }
    }
}
impl<T> From<Point2d<T>> for (T, T) {
    fn from(p: Point2d<T>) -> Self {
        (p.y, p.x)
    }
}
impl<T> From<Point2d<T>> for [T; 2] {
    fn from(p: Point2d<T>) -> Self {
        [p.y, p.x]
    }
}

// Comparisons

impl<T: Copy + Mul<Output = T> + Add<Output = T> + PartialOrd> PartialOrd for Point2d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.y * self.y + self.x * self.x).partial_cmp(&(other.y * other.y + other.x * other.x))
    }
}
impl<T: Copy + Mul<Output = T> + Add<Output = T> + Ord> Ord for Point2d<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y * self.y + self.x * self.x).cmp(&(other.y * other.y + other.x * other.x))
    }
}

// Arithmetic

impl<T: Copy + PartialOrd + Sub<Output = T>> Point2d<T> {
    pub fn abs_diff(&self, other: &Self) -> Point2d<T> {
        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        Self { y, x }
    }
}

impl<T: Copy + Into<f64>> Point2d<T> {
    pub fn mag(&self) -> f64 {
        let (y, x) = (self.y.into(), self.x.into());
        (y * y + x * x).sqrt()
    }
}

impl<T> Point2d<T> {
    pub fn new(y: T, x: T) -> Self {
        Point2d { y, x }
    }
    pub fn swap(self) -> Self {
        Point2d { y: self.x, x: self.y }
    }
}
impl<T: Default + Neg<Output=T> + PartialOrd<T>> Point2d<T> {
    pub fn abs(self) -> Self {
        let y = if self.y < T::default() { -self.y } else { self.y };
        let x = if self.x < T::default() { -self.x } else { self.x };
        Point2d { y, x }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Point3d<T> {
    pub z: T,
    pub y: T,
    pub x: T,
}

impl<T: Display> Display for Point3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(z: {}, y: {}, x: {})", self.z, self.y, self.x)
    }
}
impl<T: Debug> Debug for Point3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(z: {:?}, y: {:?}, x: {:?})", self.z, self.y, self.x)
    }
}

// Operators

impl<T: Add<Output = T>> Add<Point3d<T>> for Point3d<T> {
    type Output = Self;
    fn add(self, rhs: Point3d<T>) -> Self::Output {
        Self {
            z: self.z + rhs.z,
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}
impl<T: Add<Output = T> + Copy> Add<T> for Point3d<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            z: self.z + rhs,
            y: self.y + rhs,
            x: self.x + rhs,
        }
    }
}
impl<T: Sub<Output = T>> Sub<Point3d<T>> for Point3d<T> {
    type Output = Self;
    fn sub(self, rhs: Point3d<T>) -> Self::Output {
        Self {
            z: self.z - rhs.z,
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}
impl<T: Sub<Output = T> + Copy> Sub<T> for Point3d<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            z: self.z - rhs,
            y: self.y - rhs,
            x: self.x - rhs,
        }
    }
}
impl<T: Mul<Output = T>> Mul<Point3d<T>> for Point3d<T> {
    type Output = Self;
    fn mul(self, rhs: Point3d<T>) -> Self::Output {
        Self {
            z: self.z * rhs.z,
            y: self.y * rhs.y,
            x: self.x * rhs.x,
        }
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Point3d<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            z: self.z * rhs,
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }
}
impl<T: Div<Output = T>> Div<Point3d<T>> for Point3d<T> {
    type Output = Self;
    fn div(self, rhs: Point3d<T>) -> Self::Output {
        Self {
            z: self.z / rhs.z,
            y: self.y / rhs.y,
            x: self.x / rhs.x,
        }
    }
}
impl<T: Div<Output = T> + Copy> Div<T> for Point3d<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            z: self.z / rhs,
            y: self.y / rhs,
            x: self.x / rhs,
        }
    }
}
impl<T: AddAssign> AddAssign<Point3d<T>> for Point3d<T> {
    fn add_assign(&mut self, rhs: Point3d<T>) {
        self.z += rhs.z;
        self.y += rhs.y;
        self.x += rhs.x;
    }
}
impl<T: AddAssign + Copy> AddAssign<T> for Point3d<T> {
    fn add_assign(&mut self, rhs: T) {
        self.z += rhs;
        self.y += rhs;
        self.x += rhs;
    }
}
impl<T: SubAssign> SubAssign<Point3d<T>> for Point3d<T> {
    fn sub_assign(&mut self, rhs: Point3d<T>) {
        self.z -= rhs.z;
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}
impl<T: SubAssign + Copy> SubAssign<T> for Point3d<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.z -= rhs;
        self.y -= rhs;
        self.x -= rhs;
    }
}
impl<T: MulAssign> MulAssign<Point3d<T>> for Point3d<T> {
    fn mul_assign(&mut self, rhs: Point3d<T>) {
        self.z *= rhs.z;
        self.y *= rhs.y;
        self.x *= rhs.x;
    }
}
impl<T: MulAssign + Copy> MulAssign<T> for Point3d<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.z *= rhs;
        self.y *= rhs;
        self.x *= rhs;
    }
}
impl<T: DivAssign> DivAssign<Point3d<T>> for Point3d<T> {
    fn div_assign(&mut self, rhs: Point3d<T>) {
        self.z /= rhs.z;
        self.y /= rhs.y;
        self.x /= rhs.x;
    }
}
impl<T: DivAssign + Copy> DivAssign<T> for Point3d<T> {
    fn div_assign(&mut self, rhs: T) {
        self.z /= rhs;
        self.y /= rhs;
        self.x /= rhs;
    }
}

// Conversions

impl<T: FromStr> FromStr for Point3d<T> {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = re!(r"(-?\d+).(-?\d+).(-?\d+)");
        let Some(caps) = re.captures(s) else { return Err(r"Pattern (-?\d+).(-?\d+).(-?\d+) not recognised in input") };
        let Some(nums) = caps.iter().skip(1).map(|cap| cap.map(|cap| cap.as_str())).collect_tuple() else {
            return Err("Number of elements in string is incorrect")
        };
        let (Some(x), Some(y), Some(z)) = nums else { return Err("Number of elements in string is incorrect") };
        let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) else { return Err("Could not parse one or more elements") };
        Ok(Self { z, y, x })
    }
}

impl<T> From<(T, T, T)> for Point3d<T> {
    fn from((z, y, x): (T, T, T)) -> Self {
        Point3d { z, y, x }
    }
}
impl<T> From<[T; 3]> for Point3d<T> {
    fn from([z, y, x]: [T; 3]) -> Self {
        Point3d { z, y, x }
    }
}
impl<T> From<Point3d<T>> for (T, T, T) {
    fn from(p: Point3d<T>) -> Self {
        (p.z, p.y, p.x)
    }
}
impl<T> From<Point3d<T>> for [T; 3] {
    fn from(p: Point3d<T>) -> Self {
        [p.z, p.y, p.x]
    }
}

// Comparisons

impl<T: Copy + Mul<Output = T> + Add<Output = T> + PartialOrd> PartialOrd for Point3d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.z * self.z + self.y * self.y + self.x * self.x)
            .partial_cmp(&(other.z * other.z + other.y * other.y + other.x * other.x))
    }
}
impl<T: Copy + Mul<Output = T> + Add<Output = T> + Ord> Ord for Point3d<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.z * self.z + self.y * self.y + self.x * self.x)
            .cmp(&(other.z * other.z + other.y * other.y + other.x * other.x))
    }
}

// Arithmetic

impl<T: Copy + PartialOrd + Sub<Output = T>> Point3d<T> {
    pub fn abs_diff(&self, other: &Self) -> Point3d<T> {
        let z = if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };
        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        Self { z, y, x }
    }
}

impl<T: Copy + Into<f64>> Point3d<T> {
    pub fn mag(&self) -> f64 {
        let (z, y, x) = (self.z.into(), self.y.into(), self.x.into());
        (z * z + y * y + x * x).sqrt()
    }
}

impl<T> Point3d<T> {
    pub fn new(z: T, y: T, x: T) -> Self {
        Point3d { z, y, x }
    }
}

macro_rules! pt {
    () => { Point2d::<i32>::default() };
    ($y: expr, $x: expr) => {
        Point2d::new($y, $x)
    };
    ($z: expr, $y: expr, $x: expr) => {
        Point3d::new($z, $y, $x)
    };
}
pub(crate) use pt;
