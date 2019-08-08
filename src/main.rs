


//mod sse2;

mod math {
    use std::arch::x86_64::*;

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

}




fn main() {
    let v1 : [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    let v2 : [f32; 4] = [5.0, 5.0, 10.0, 5.0];

    let v3 = math::LoadFloat4(&math::Float4(v1));
    let v4 = math::LoadFloat4(&math::Float4(v2));

    let v5 = math::Vector2Dot(&v3, &v4);

    let mut v6 = math::Float2([0_f32; 2]);
    math::StoreFloat2(&mut v6, &v5);

    println!("{:?}", v4);
    println!("{:?}", v6);
}