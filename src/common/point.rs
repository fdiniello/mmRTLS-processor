use std::{
    fmt::{Display, Formatter},
    ops,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    pub fn is_valid(&self) -> bool {
        return !self.x.is_nan() && !self.y.is_nan();
    }
    pub fn module(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn phase(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }
    pub fn distance(a: &Point, b: &Point) -> f64 {
        (a - b).module()
    }
    pub fn distance_to(&self, other: &Point) -> f64 {
        (self - other).module()
    }
    pub fn to_versor(&self) -> Option<Point> {
        if self.x == 0.0 && self.y == 0.0 {
            None
        } else {
            Some(self / self.module())
        }
    }
    pub fn rotate_by(&mut self, α: f64) {
        let m = self.module();
        let (sin, cos) = f64::sin_cos(self.phase() + α);
        self.x = m * cos;
        self.y = m * sin;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:.2},{:.2})", &self.x, &self.y)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::Add<&Point> for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}
impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::Mul<f64> for &Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        *self = Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ops::Div<f64> for &Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
