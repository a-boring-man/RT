use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    p: [f64; 3],
}

pub fn sqr(n: f64) -> f64{
    n * n
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {p: [x, y, z]}
    }

    pub fn length2(&self) -> f64 {
        sqr(self.x()) + sqr(self.y()) + sqr(self.z())
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self) -> f64 {
        let l: f64 = self.length();
        *self /= l;
        l
    }

    pub fn normalized(&self) -> Vec3 {
        let mut v: Vec3 = self.clone();
        v.normalize();
        v
    }

    pub fn x(&self) -> f64 {
        self.p[0]
    }
    
    pub fn r(&self) -> f64 {
        self.p[0]
    }

    pub fn y(&self) -> f64 {
        self.p[1]
    }
    
    pub fn g(&self) -> f64 {
        self.p[1]
    }
    pub fn z(&self) -> f64 {
        self.p[2]
    }
    
    pub fn b(&self) -> f64 {
        self.p[2]
    }

    pub fn set(&mut self, x: f64, y:f64, z: f64) {
        self.p[0] = x;
        self.p[1] = y;
        self.p[2] = z;
    }

}

/**********************
 * Overload implementation section
 **********************/

/* Implement the + operator for Vec3 */
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl<'a> ops::Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl ops::Add<& Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: & Vec3) -> Self::Output {
        Vec3 { p: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl ops::Add<Vec3> for & Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { p: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.p[0] += rhs.x();
        self.p[1] += rhs.y();
        self.p[2] += rhs.z();
    }
}
impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.p[0] += rhs.x();
        self.p[1] += rhs.y();
        self.p[2] += rhs.z();
    }
}

/* Implement the - operator for Vec3 */
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl<'a> ops::Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl ops::Sub<& Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: & Vec3) -> Self::Output {
        Vec3 { p: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl ops::Sub<Vec3> for & Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { p: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.p[0] -= rhs.x();
        self.p[1] -= rhs.y();
        self.p[2] -= rhs.z();
    }
}
impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.p[0] -= rhs.x();
        self.p[1] -= rhs.y();
        self.p[2] -= rhs.z();
    }
}

/* Implement the * operator for Vec3 */
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl<'a> ops::Mul<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl ops::Mul<& Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: & Vec3) -> Self::Output {
        Vec3 { p: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl ops::Mul<Vec3> for & Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { p: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { p: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}
impl ops::Mul<f64> for & Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { p: [ self.x() * rhs, self.y() * rhs, self.z() * rhs ] }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { p: [ self * rhs.x(), self * rhs.y(), self * rhs.z() ] }
    }
}
impl ops::Mul<& Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: & Vec3) -> Self::Output {
        Vec3 { p: [ self * rhs.x(), self * rhs.y(), self * rhs.z() ] }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.p[0] *= rhs;
        self.p[1] *= rhs;
        self.p[2] *= rhs;
    }
}

/* Implement the / operator for Vec3 */
impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl<'a> ops::Div<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 { p: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl ops::Div<& Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: & Vec3) -> Self::Output {
        Vec3 { p: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl ops::Div<Vec3> for & Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { p: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let k: f64 = 1.0/rhs;
        Vec3 { p: [self.x() * k, self.y() * k, self.z() * k] }
    }
}
impl ops::Div<f64> for & Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        let k = 1.0 / rhs;
        Vec3 { p: [self.x() * k, self.y() * k, self.z() * k] }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let l: f64 = 1.0 / rhs;
        self.p[0] *= l;
        self.p[1] *= l;
        self.p[2] *= l;
    }
}

/* Implement negate opperator for the vector struct */
impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output{
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}