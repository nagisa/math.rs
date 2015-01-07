// Need ilogb
// /// Floating-point remainder of dividing l by r.
// ///
// /// The return value is x - n * y, where n is the quotient of x / y, rounded toward zero to an
// /// integer.
// pub extern fn fmodf(l: f32, r: f32) -> f32 {
//     let (mut lbits, mut rbits) = (l.as_bits(), r.as_bits());
//     let (rabs, labs) = (l.abs().as_bits(), r.abs().as_bits());
//
//     if r == 0 || !l.is_finite() || !r.is_finite() {
//         return (l * r) / (r * l);
//     } else if labs < rabs {
//         return l;
//     } else if rabs == labs {
//         return if lbits >> (31 as uint) != 0 { -0.0 } else { 0.0 };
//     }
//
//
//
//
//
// }
//
// /// Floating-point remainder of dividing l by r.
// ///
// /// The return value is x - n * y, where n is the quotient of x / y, rounded toward zero to an
// /// integer.
// pub extern fn fmod(l: f64, r: f64) -> f64 {
//
// }
