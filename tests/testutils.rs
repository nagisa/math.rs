use std::{f32, f64};

// #[allow(dead_code)]
// pub const F32_MIN_SUBNORM: f32 = 1.4E-45;
// #[allow(dead_code)]
// pub const F64_MIN_SUBNORM: f64 = 4.94065645841246544176568792868E-324;


pub trait AsBits {
    type Output;
    fn as_bits(&self) -> Self::Output;
    fn print_bits(&self);
}
impl<A: AsBits, B: AsBits> AsBits for (A, B) {
    type Output = (A::Output, B::Output);
    fn as_bits(&self) -> Self::Output { (self.0.as_bits(), self.1.as_bits()) }
    fn print_bits(&self) { self.0.print_bits(); self.1.print_bits(); }
}
impl AsBits for f32 {
    type Output = u32;
    fn as_bits(&self) -> Self::Output { unsafe { *(self as *const _ as *const _) } }
    fn print_bits(&self) { println!("BITS {:08x}", self.as_bits()); }
}
impl AsBits for f64 {
    type Output = u64;
    fn as_bits(&self) -> Self::Output { unsafe { *(self as *const _ as *const _) } }
    fn print_bits(&self) { println!("BITS {:08x}", self.as_bits()); }
}
impl AsBits for i32 {
    type Output = u32;
    fn as_bits(&self) -> Self::Output { *self as _ }
    fn print_bits(&self) { println!("BITS {:08x}", self.as_bits()); }
}
impl AsBits for i64 {
    type Output = u64;
    fn as_bits(&self) -> Self::Output { *self as _ }
    fn print_bits(&self) { println!("BITS {:08x}", self.as_bits()); }
}

macro_rules! check {
    ($name: ident ~ |$($id: ident: $arg:ty),*| -> $ret: ty $body: block, $cwrap: expr, [$(($($extra_case: expr),*)),*]) => {
        #[test]
        fn $name() {
            #[cfg(not(windows))]
            const LIBM: &'static str = "libm.so.6";
            fn test($($id: $arg),*) -> bool {
                let libm = ::libloading::Library::new(LIBM).expect("could not open libm");
                let libmfn = unsafe {
                    // Sort of dangerous lol
                    let symbol: ::libloading::Symbol<_> =
                        libm.get(stringify!($name).as_bytes()).expect("symbol exists");
                    symbol
                };

                let our_ans: $ret = $body;
                let libm_ans = ($cwrap)(*libmfn, $($id),*);
                let ret = our_ans.as_bits() == libm_ans.as_bits();
                if !ret {
                    println!("OUR: {:?} ", our_ans);
                    our_ans.print_bits();
                    println!("LIBM: {:?} ", libm_ans);
                    libm_ans.print_bits();
                }
                ret
            }

            $(
            assert!(test($($extra_case),*), "{} failed with arguments {:?}",
                    stringify!($name), ($($extra_case),*));
            )*

            ::quickcheck::quickcheck(test as fn($($arg),*) -> _);
        }
    };
    ($name: ident ~ |$($id: ident: $arg:ty),*| -> $ret: ty $body: block, [$(($($extra_case: expr),*)),*]) => {
        check!($name ~ |$($id: $arg),*| -> $ret $body, |f: unsafe extern fn($($arg),*)->$ret, $($id),*| -> $ret { unsafe { f($($id),*) } }, [$(($($extra_case),*)),*]);
    };
    ($name: ident ~ |$($id: ident: $arg:ty),*| -> $ret: ty $body: block) => {
        check!($name ~ |$($id: $arg),*| -> $ret $body, []);
    }
}
