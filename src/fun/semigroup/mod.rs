use crate::fun::Semigroup;
use std::boxed::Box;

impl<'a> Semigroup for &'a str {
  fn mappend(self: &Self, other: &Self) -> Self {
    Box::leak(format!("{}{}",*self, *other).into_boxed_str())
  }
}

impl Semigroup for i8 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl Semigroup for i32 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl Semigroup for i64 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl Semigroup for u8 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl Semigroup for u32 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl Semigroup for u64 {
  fn mappend(self: &Self, other: &Self) -> Self {
    self + other
  }
}

impl<A: Semigroup> Semigroup for Option<A> {
  fn mappend(self: &Self, other: &Self) -> Self {
    match (self, other) {
      (&None, b) => b.clone(),
      (a, &None) => a.clone(),
      (&Some(ref a), &Some(ref b)) => Some(a.mappend(b)),
    }
  }
}

impl <A: Semigroup> Semigroup for Box<A> {
  fn mappend(self: &Self, other: &Self) -> Self {
    Box::new(self.as_ref().mappend(other.as_ref()))
  }
}

#[cfg(test)]
mod test {
  use crate::fun::Semigroup;

  #[test]
  fn i32() {
    assert_ne!(3, 1.mappend(&3));
    assert_eq!(4, 1.mappend(&3));
  }

  #[test]
  fn option_i32_both() {
    assert_eq!(Some(3), Some(1).mappend(&Some(2)));
  }

  #[test]
  fn option_i32_a() {
    assert_eq!(Some(1), Some(1).mappend(&None));
  }

  #[test]
  fn option_i32_b() {
    assert_eq!(Some(2), (None::<i32>).mappend(&Some(2)));
  }

  #[test]
  fn option_i32_none() {
    assert_eq!(None::<i32>, (None::<i32>).mappend(&None::<i32>));
  }

  #[test]
  fn box_u8() {
    let res: u8 = 5;
    let v1: u8 = 2;
    let v2: u8 = 3;
    assert_eq!(Box::new(res), Box::new(v1).mappend(&Box::new(v2)));
  }

 #[test]
  fn box_u64_not() {
    let res: u64 = 6;
    let v1: u64 = 2;
    let v2: u64 = 3;
    assert_ne!(Box::new(res), Box::new(v1).mappend(&Box::new(v2)));
  }

  #[test]
  fn str_test() {
    assert_eq!("hello, world", "hello".mappend(&", world"));
  }

}