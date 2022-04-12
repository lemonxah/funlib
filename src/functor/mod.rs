//!
//! Functor implementations and tests
//!

use crate::Functor;
use std::vec::Vec;
use std::boxed::Box;
use std::rc::Rc;

impl<A,B> Functor<B> for Option<A> {
  fn fmap<F>(&self, f: F) -> Option<B> where F: Fn(&A) -> B {
    match *self{
      Some(ref a) => Some(f(a)),
      None => None,
    }
  }
}

impl<A,B> Functor<B> for Box<A> {
  fn fmap<F>(&self, f: F) -> Box<B> where F: Fn(&A) -> B {
    Box::new(f(self))
  }
}

impl<A,B> Functor<B> for Rc<A> {
  fn fmap<F>(&self, f: F) -> Rc<B> where F: Fn(&A) -> B {
    Rc::new(f(self))
  }
}

impl<A,B> Functor<B> for Vec<A> {
  fn fmap<F>(&self, f: F) -> Vec<B> where F: Fn(&A) -> B {
    self.iter().map(f).collect()
  }
}

#[cfg(test)]
mod test {
  use crate::Functor;
  use std::rc::Rc;
  use std::boxed::Box;

  #[cfg(test)]
  mod laws {
    use crate::Functor;
    use std::rc::Rc;
    use std::boxed::Box;

    fn id<A>(v: A) -> A { v }
    macro_rules! laws {
      ($t: ident, $v: expr, $f: expr, $f2: expr) => {
        #[allow(non_snake_case)]
        #[test]
        fn $t() {
          assert_eq!($v, $v.fmap(|x| *id(x)));
          assert_eq!($v.fmap(|x| compose!($f, $f2)(x)), $v.fmap($f).fmap($f2))
        }
      };
    }
    laws!(Option, Some(5i32), |x| x + 2, |x| x * 5);
    laws!(Box, Box::new(5i32), |x| x + 2, |x| x * 5);
    laws!(Rc, Rc::new(5i32), |x| x + 2, |x| x * 5);
    laws!(Vec, vec![1,2,3,4], |x| x + 2, |x| x * 5);
  }

  #[test]
  fn option() {
    let some = Some(1);
    let none: Option::<i32> = None;
    assert_eq!(None::<i32>, none.fmap(|x| x + 1));
    assert_eq!(Some(2), some.fmap(|x| x + 1));
  }

  #[test]
  fn box_() {
    let ax = Box::new(1);
    let bx = ax.fmap(|x| x + 1);
    let cx = ax.fmap(|x| x + 2);
    assert_eq!(Box::new(2), bx);
    assert_eq!(Box::new(3), cx);
  }

  #[test]
  fn rc() {
    let ax = Rc::new(1);
    let bx = ax.fmap(|x| x + 1);
    let cx = ax.fmap(|x| x + 2);
    assert_eq!(Rc::new(2), bx);
    assert_eq!(Rc::new(3), cx);
  }


  #[test]
  fn vec_() {
    let ax = vec![1,2,3];
    let bx = ax.fmap(|x| x + 1);
    assert_eq!(vec![2,3,4], bx);
  }

}
