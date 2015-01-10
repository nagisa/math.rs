// Read sin.rs for implementation details.
//
// Tailor series for cos(x) look like this:
//
//     x^2    x^4     x^6       x^8         x^10        x^12
// 1 - --- + ----- - ------ + -------- - ---------- + ---------- - …
//      2!     4!      6!        8!          10!         12!

const D2  : f64 = -0.5;
const D4  : f64 =  4.166666666666666666666666666666E-2;
const D6  : f64 = -1.388888888888888888888888888888E-3;
const D8  : f64 =  2.480158730158730158730158730158E-5;
const D10 : f64 = -2.755731922398589065255731922398E-7;
const D12 : f64 =  2.087675698786809897921009032120E-9;

pub fn _cos(i: f64) -> f64 {
    let i2 = i * i;
    let i4 = i2 * i2;
    let i6 = i4 * i2;
    return 1.0 + (i * D2 * i) + (i2 * D4 * i2) + (i4 * D6 * i2) + (i4 * D8 * i4)
               + (i6 * D10 * i4) + (i6 * D12 * i6);
}
