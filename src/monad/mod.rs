//!
//! Monad implementations and tests
//!

use crate::Monad;
use std::boxed::Box;
use std::rc::Rc;

impl<A, B> Monad<B> for Option<A> {
  fn bind<F>(&self, mut f: F) -> Option<B> where F: FnMut(&A) -> Option<B> {
    match *self {
      Some(ref a) => f(a),
      None => None,
    }
  }
}

impl<A, B> Monad<B> for Box<A> {
  fn bind<F>(&self, mut f: F) -> Box<B> where F: FnMut(&A) -> Box<B> {
    f(self)
  }
}

impl<A, B> Monad<B> for Rc<A> {
  fn bind<F>(&self, mut f: F) -> Rc<B> where F: FnMut(&A) -> Rc<B> {
    f(self)
  }
}

impl<A, B> Monad<B> for Vec<A> {
  fn bind<F>(&self, f: F) -> Vec<B> where F: FnMut(&A) -> Vec<B> {
    self.iter().flat_map(f).collect()
  }
}

#[cfg(test)]
mod test {
  use crate::Monad;
  use crate::Functor;
  use crate::Applicative;
  use std::rc::Rc;

  #[test]
  fn option() {
    assert_eq!(Some("hello"), Some(5).fmap(|_i| "hello"));
    assert_eq!(Option::<i32>::pure_(10), Some(5).bind(|i| Some(i * 1)).fmap(|i| i * 2));
    assert_eq!(Option::<i32>::pure_(10), Some(5).bind(|i| Some(i * 2)));
  }

  #[test]
  fn box_() {
    assert_eq!(Box::pure_(10), Box::pure_(5).bind(|i| Box::new(i * 2)));
  }

  #[test]
  fn rc() {
    assert_eq!(Rc::pure_(10), Rc::pure_(5).bind(|i| Rc::new(i * 2)));
  }

  #[test]
  fn vec() {
    let mut vec = Vec::pure_(1);
    vec.push(2);
    vec.push(3);
    assert_eq!(vec![1,2,2,4,3,6], vec.bind(|x| vec![x * 1, x * 2]));
  }
}
