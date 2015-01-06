use core::intrinsics::transmute;

pub trait Abs {
    /// Absolute value of the IEEE 754 floating point number.
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> f32 {
        unsafe {
            let bits : u32 = transmute(self);
            transmute(bits & 0x7fff_ffff)
        }
    }
}

impl Abs for f64 {
    fn abs(self) -> f64 {
        unsafe {
            let bits : u64 = transmute(self);
            transmute(bits & 0x7fff_ffff_ffff_ffff)
        }
    }
}

#[test]
fn abs_f32() {
    assert_eq!(0.1f32.abs(), 0.1f32);
    assert_eq!((-0.1f32).abs(), 0.1f32);
    assert_eq!((-2f32).abs(), 2f32);
    assert_eq!(2f32.abs(), 2f32);
}

#[test]
fn abs_f64() {
    assert_eq!(0.1f64.abs(), 0.1f64);
    assert_eq!((-0.1f64).abs(), 0.1f64);
    assert_eq!((-2f64).abs(), 2f64);
    assert_eq!(2f64.abs(), 2f64);
}
