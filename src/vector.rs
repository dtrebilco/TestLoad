use std::{mem::size_of, ops::Index};

#[macro_export]
macro_rules! static_assert {
    ($condition:expr) => {
        const _: () = core::assert!($condition);
    };
}

macro_rules! vec_ops {
    ($name:ident) => {
        impl std::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: $name) {
                *self = *self + rhs;
            }
        }

        impl std::ops::SubAssign<$name> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs;
            }
        }

        impl std::ops::MulAssign<f32> for $name {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl std::ops::DivAssign<f32> for $name {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    };
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(PartialEq, Copy, Clone)]
pub struct vec2 {
    pub x: f32,
    pub y: f32,
}
static_assert!(size_of::<vec2>() == 8);

pub const fn vec2(x: f32, y: f32) -> vec2 {
    vec2 { x, y }
}

impl std::ops::Mul<f32> for vec2 {
    type Output = vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for vec2 {
    type Output = vec2;

    fn div(self, rhs: f32) -> Self::Output {
        vec2(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Add<vec2> for vec2 {
    type Output = vec2;

    fn add(self, rhs: vec2) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<vec2> for vec2 {
    type Output = vec2;

    fn sub(self, rhs: vec2) -> Self::Output {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Neg for vec2 {
    type Output = vec2;

    fn neg(self) -> Self::Output {
        vec2(-self.x, -self.y)
    }
}
vec_ops!(vec2);

// -------------------------------------------------------------------------------------------

#[repr(C)]
#[allow(non_snake_case)]
#[derive(PartialEq, Copy, Clone)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
static_assert!(size_of::<vec3>() == 12);

pub const fn vec3(x: f32, y: f32, z: f32) -> vec3 {
    vec3 { x, y, z }
}

pub fn dot(a: &vec3, b: &vec3) -> f32 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

pub fn length_squared(vec: &vec3) -> f32 {
    dot(vec, vec)
}

pub fn length(vec: &vec3) -> f32 {
    length_squared(vec).sqrt()
}

pub fn normalize(vec: &vec3) -> vec3 {
    *vec / length(vec)
}

impl std::ops::Mul<f32> for vec3 {
    type Output = vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f32> for vec3 {
    type Output = vec3;

    fn div(self, rhs: f32) -> Self::Output {
        vec3(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Add<vec3> for vec3 {
    type Output = vec3;

    fn add(self, rhs: vec3) -> Self::Output {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<vec3> for vec3 {
    type Output = vec3;

    fn sub(self, rhs: vec3) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Neg for vec3 {
    type Output = vec3;

    fn neg(self) -> Self::Output {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

vec_ops!(vec3);

// -------------------------------------------------------------------------------------------

#[repr(C)]
#[allow(non_snake_case)]
#[derive(PartialEq, Copy, Clone)]
pub struct vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
static_assert!(size_of::<vec4>() == 16);

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> vec4 {
    vec4 { x, y, z, w }
}

pub fn lerp(x: &vec4, y: &vec4, a: f32) -> vec4 {
    *x + ((*y - *x) * a)
}

impl std::ops::Mul<f32> for vec4 {
    type Output = vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        vec4(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Div<f32> for vec4 {
    type Output = vec4;

    fn div(self, rhs: f32) -> Self::Output {
        vec4(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl std::ops::Add<vec4> for vec4 {
    type Output = vec4;

    fn add(self, rhs: vec4) -> Self::Output {
        vec4(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Sub<vec4> for vec4 {
    type Output = vec4;

    fn sub(self, rhs: vec4) -> Self::Output {
        vec4(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Neg for vec4 {
    type Output = vec4;

    fn neg(self) -> Self::Output {
        vec4(-self.x, -self.y, -self.z, -self.w)
    }
}

vec_ops!(vec4);
