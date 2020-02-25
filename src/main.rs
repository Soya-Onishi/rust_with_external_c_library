#[repr(C)]
union SoftFloat32 {
    i: u32,
    f: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct float32_t { v: u32 }

#[link(name="softfloat")]
extern "C" {
    fn f32_add(a: float32_t, b: float32_t) -> float32_t;
    fn f32_mul(a: float32_t, b: float32_t) -> float32_t;
}

fn main() {
    let a: float32_t = f32_to_float32_t(0.1);
    let b: float32_t = f32_to_float32_t(3.5);

    let add = unsafe { f32_add(a, b) };
    let mul = unsafe { f32_mul(a, b) };

    let add = float32_t_to_f32(add);
    let mul = float32_t_to_f32(mul);

    println!("add: {}, mul: {}", add, mul);
}

fn f32_to_float32_t(f: f32) -> float32_t {
    let sf32 = SoftFloat32 { f };
    float32_t { v: unsafe { sf32.i } }
}

fn float32_t_to_f32(f: float32_t) -> f32 {
    let sf32 = SoftFloat32 { i: f.v };
    unsafe { sf32.f }
}
