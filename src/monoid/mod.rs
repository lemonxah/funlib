//!
//! Monoid implementations and tests
//! 

use crate::Monoid;

macro_rules! monoid {
  ($t:ident, $v:expr) => {
    impl Monoid for $t {
      fn mempty() -> Self { $v }
    }
  }
}

monoid!(i8, 0);
monoid!(i16, 0);
monoid!(i32, 0);
monoid!(i64, 0);
monoid!(u8, 0);
monoid!(u16, 0);
monoid!(u32, 0);
monoid!(u64, 0);
monoid!(f32, 0.0);
monoid!(f64, 0.0);

impl<A: Monoid> Monoid for Option<A> {
  fn mempty() -> Self {
    None::<A>
  }
}

impl<A: Monoid> Monoid for Box<A> {
  fn mempty() -> Self {
    Box::new(A::mempty())
  }
}

impl<'a> Monoid for &'a str {
  fn mempty() -> Self {
    ""
  }
}

#[cfg(test)]
mod test {
  use crate::Monoid;

  #[cfg(test)]
  mod laws {
    use crate::Monoid;
    use crate::Semigroup;

    macro_rules! laws {
      ($t: ident, $v: expr, $v2: expr, $v3: expr) => {
        #[allow(non_snake_case)]
        #[test]
        fn $t() {
          assert_eq!($v, $t::mempty().mappend(&$v));
          assert_eq!($v, $v.mappend(&$t::mempty()));
          assert_eq!($v.mappend(&$v2.mappend(&$v3)),($v.mappend(&$v2)).mappend(&$v3));
        }
      };
    }
    macro_rules! laws2 {
      ($t: ident, $t2: ident, $v: expr, $v2: expr, $v3: expr) => {
        #[allow(non_snake_case)]
        #[test]
        fn $t() {
          assert_eq!($v, $t::<$t2>::mempty().mappend(&$v));
          assert_eq!($v, $v.mappend(&$t::mempty()));
          assert_eq!($v.mappend(&$v2.mappend(&$v3)),($v.mappend(&$v2)).mappend(&$v3));
        }
      };
    }    
    laws!(i32, 5i32, 6i32, 10i32);
    laws!(u64, 5u64, 6u64, 10u64);
    laws2!(Option, u8, Some(5u8), Some(6u8), Some(10u8));
    laws2!(Box, i64, Box::new(5i64), Box::new(6i64), Box::new(10i64));
  }

  #[test]
  fn i32() {
    assert_eq!(0, i32::mempty());
  }

  #[test]
  fn u8() {
    assert_eq!(0, u8::mempty());
  }

  #[test]
  fn option_i64() {
    assert_eq!(None::<i64>, Option::<i64>::mempty());
  }

  #[test]
  fn _box_i8() {
    assert_eq!(Box::new(0), Box::<i8>::mempty());
  }

  #[test]
  fn _box_str() {
    assert_eq!(Box::new(""), Box::<&str>::mempty());
  }

}