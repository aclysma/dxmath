/*
use std::arch::x86_64::*;

//mod sse2;

#[derive(Debug)]
pub struct Float2(pub [f32;2]);

impl Float2 {
    fn x(&self) -> f32 { self.0[0] }
    fn y(&self) -> f32 { self.0[1] }

    fn x_mut(&mut self) -> &mut f32 { &mut self.0[0] }
    fn y_mut(&mut self) -> &mut f32 { &mut self.0[1] }
}

#[derive(Debug)]
pub struct Float3(pub [f32;3]);

impl Float3 {
    fn x(&self) -> f32 { self.0[0] }
    fn y(&self) -> f32 { self.0[1] }
    fn z(&self) -> f32 { self.0[2] }

    fn x_mut(&mut self) -> &mut f32 { &mut self.0[0] }
    fn y_mut(&mut self) -> &mut f32 { &mut self.0[1] }
    fn z_mut(&mut self) -> &mut f32 { &mut self.0[2] }
}

#[derive(Debug)]
pub struct Float4(pub [f32;4]);

impl Float4 {
    fn x(&self) -> f32 { self.0[0] }
    fn y(&self) -> f32 { self.0[1] }
    fn z(&self) -> f32 { self.0[2] }
    fn w(&self) -> f32 { self.0[3] }

    fn x_mut(&mut self) -> &mut f32 { &mut self.0[0] }
    fn y_mut(&mut self) -> &mut f32 { &mut self.0[1] }
    fn z_mut(&mut self) -> &mut f32 { &mut self.0[2] }
    fn w_mut(&mut self) -> &mut f32 { &mut self.0[3] }
}

/*
pub struct Int2 {
    x: i32,
    y: i32
}

pub struct Int3 {
    x: i32,
    y: i32,
    z: i32
}

pub struct Int4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}


pub struct UInt2 {
    x: u32,
    y: u32
}

pub struct UInt3 {
    x: u32,
    y: u32,
    z: u32
}

pub struct UInt4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32
}



pub struct Float3x3 {

}

pub struct Float4x3 {

}

pub struct Float3x4 {

}

pub struct Float4x4 {

}
*/

#[derive(Debug)]
pub struct Vector_f32(pub [f32; 4]);
#[derive(Debug)]
pub struct Vector_m128(pub __m128);


#[derive(Debug)]
pub struct Matrix_f32(pub [[f32; 4]; 4]);
#[derive(Debug)]
pub struct Matrix_m128(pub [Vector_m128; 4]);

pub use Vector_m128 as Vector;
pub use Matrix_m128 as Matrix;

#[inline(always)]
pub fn LoadFloat2(src: &Float2) -> Vector {
    unsafe { Vector(_mm_load_ps(src.0.as_ptr())) }
}

#[inline(always)]
pub fn LoadFloat3(src: &Float3) -> Vector {
    unsafe { Vector(_mm_load_ps(src.0.as_ptr())) }
}

#[inline(always)]
pub fn LoadFloat4(src: &Float4) -> Vector {
    unsafe { Vector(_mm_load_ps(src.0.as_ptr())) }
}

#[inline(always)]
pub fn StoreFloat2(dst: &mut Float2, src: &Vector) {
    unsafe { _mm_storeu_ps(dst.0.as_mut_ptr(), src.0) }
}

#[inline(always)]
pub fn StoreFloat3(dst: &mut Float3, src: &Vector) {
    unsafe { _mm_storeu_ps(dst.0.as_mut_ptr(), src.0) }
}

#[inline(always)]
pub fn StoreFloat4(dst: &mut Float4, src: &Vector) {
    unsafe { _mm_storeu_ps(dst.0.as_mut_ptr(), src.0) }
}

pub fn Vector2Dot(v1: &Vector, v2: &Vector) -> Vector {
    Vector(unsafe { _mm_dp_ps(v1.0, v2.0, 0x3f) })
}


*/
