#![allow(dead_code)]

mod math;

fn main() {
    let v1 : [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    let v2 : [f32; 4] = [5.0, 5.0, 10.0, 5.0];

    let v3 = math::load_float4(&math::Float4(v1));
    let v4 = math::load_float4(&math::Float4(v2));

    let v5 = math::vector2_dot(&v3, &v4);

    let mut v6 = math::Float2([0_f32; 2]);
    math::store_float2(&mut v6, &v5);

    println!("{:?}", v4);
    println!("{:?}", v6);
}
