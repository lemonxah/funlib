//!
//! Macros for use with the functional library
//!

/// Derive HKT macro to create Higer Kinded Types
#[macro_export]
macro_rules! derive_hkt {
  ($t:ident) => {
    impl<B, C> HKT<C> for $t<B> {
      type A = B;
      type M = $t<C>;
    }
    impl<'a, B: 'a> HKST<'a, B> for $t<B> {
      type A = &'a B;
      type M = $t<&'a B>;
    }
  };
}

/// Compose functions
///
/// # Examples
///
// / ```
// / # #[macro_use] extern crate funlib;
// / # fn main() {
// / fn add1(i: i32) -> i32 { i + 1 }
// / fn double(i: i32) -> i32 { i * 2 }
// /
// / let c = compose!(add1, double);
// / assert_eq!(4, c(1));
// / # }
// / ```
#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        crate::macros::compose_two($head, compose!($($tail),+))
    };
}
/// Used in compose macro to compose functions together
pub fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(A) -> B, G: Fn(B) -> C {
    move |x| g(f(x))
}
