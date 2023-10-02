#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub use Vec3 as Point3;

use crate::{util::random_float, axis::{AxisIndex, Axis}};

impl Vec3 {
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random(min: Option<f32>, max: Option<f32>) -> Self {
        Self {
            x: random_float(min, max),
            y: random_float(min, max),
            z: random_float(min, max),
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x < f32::EPSILON && self.y < f32::EPSILON && self.z < f32::EPSILON
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(Some(-1.0), Some(1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    return  if on_unit_sphere.dot(normal) > 0.0 {on_unit_sphere} else {-on_unit_sphere}
}

pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    *vec - 2.0 * vec.dot(normal) * *normal
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat:f32) -> Vec3 {
    let cos_theta = -uv.dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            x: random_float(Some(-1.0), Some(1.0)), 
            y: random_float(Some(-1.0), Some(1.0)), 
            z: 0.0
        };
        if p.length_squared() < 1.0 {return p;}
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0/rhs;
    }
}

impl AxisIndex<f32> for Vec3 {
    fn axis(&self, axis: &crate::axis::Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

#[derive(Default)]
pub struct Vec2<T> 
where T: std::ops::Add, 
{
    pub width: T,
    pub height: T,
}
