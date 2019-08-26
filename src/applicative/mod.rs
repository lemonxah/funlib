use crate::Applicative;
use crate::HKT;
use std::rc::Rc;

impl<A, B> Applicative<B> for Option<A> {
  fn pure_(b: B) -> <Self as HKT<B>>::M {
    Some(b)
  }

  fn ap<F>(&self, of: <Self as HKT<F>>::M) -> Option<B> where  F: Fn(&A) -> B {
    match (self, of) {
      (&Some(ref a), Some(ref f)) => Some(f(a)),
      (_, _) => None
    }
  }
}

impl<A, B> Applicative<B> for Box<A> {
  fn pure_(b: B) -> <Self as HKT<B>>::M {
    Box::new(b)
  }

  fn ap<F>(&self, of: <Self as HKT<F>>::M) -> Box<B> where F: Fn(&A) -> B {
    Box::new(of(self))
  }
}

impl<A, B> Applicative<B> for Rc<A> {
  fn pure_(b: B) -> <Self as HKT<B>>::M {
    Rc::new(b)
  }

  fn ap<F>(&self, of: <Self as HKT<F>>::M) -> Rc<B> where F: Fn(&A) -> B {
    Rc::new(of(self))
  }
}

impl<A, B> Applicative<B> for Vec<A> {
  fn pure_(b: B) -> <Self as HKT<B>>::M {
    vec![b]
  }

  fn ap<F>(&self, of: <Self as HKT<F>>::M) -> Vec<B> where F: Fn(&A) -> B {
    let mut result = vec![];
    for (i,f) in of.into_iter().enumerate() {
      result.push(f(&self[i]))
    }
    result
  }
}

#[cfg(test)]
mod test {
  use crate::Applicative;
  use std::rc::Rc;

  #[test]
  fn option() {
    let f: &dyn Fn(&i32) -> i32 = &|x| x * 2;
    assert_eq!(Some(4), Some(2).ap(Some(f)));
  }

  #[test]
  fn box_() {
    let f: &dyn Fn(&i32) -> i32 = &|x| x * 2;
    assert_eq!(Box::new(4), Box::new(2).ap(Box::new(f)));
  }

  #[test]
  fn rc() {
    let f: &dyn Fn(&i32) -> i32 = &|x| x * 2;
    assert_eq!(Rc::new(4), Rc::new(2).ap(Rc::new(f)));
  }

  #[test]
  fn vec() {
    let f1: &dyn Fn(&i32) -> i32 = &|x| x + 6;
    let f2: &dyn Fn(&i32) -> i32 = &|x| x * x;
    let f3: &dyn Fn(&i32) -> i32 = &|x| x * x * x;
    assert_eq!(vec![7,4,27], vec![1,2,3].ap(vec![f1,f2,f3]))
  }
}