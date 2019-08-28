pub mod functor;
pub mod applicative;
pub mod monad;
pub mod semigroup;
pub mod monoid;
pub mod foldable;
use std::rc::Rc;

pub trait HKT<B> {
  type A; // Current type
  type M; // Type A swapped with B
}

pub trait HKST<'a, B> {
  type A;
  type M;
}

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

derive_hkt!(Vec);
derive_hkt!(Option);
derive_hkt!(Box);
derive_hkt!(Rc);

pub trait Functor<B>: HKT<B> {
  fn fmap<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> B;
}

pub trait Applicative<B>: Functor<B> {
  fn pure_(value: B) -> Self::M where Self: HKT<B, A=B>;
  fn ap<F>(&self, f: <Self as HKT<F>>::M) -> <Self as HKT<B>>::M
      where F: Fn(&<Self as HKT<B>>::A) -> B, Self:HKT<F>;
}

pub trait Monad<B>: Applicative<B> {
  fn bind<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> Self::M;
  fn return_(v: B) -> Self::M where Self: HKT<B, A=B> {
    Self::pure_(v)
  }
  fn fmap<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> B, Self: HKT<B, A=B> {
    Self::bind(self, |a| Self::pure_(f(a)))
  }
}

pub trait Semigroup: Clone {
  fn mappend(&self, other: &Self) -> Self;
}

pub trait Monoid: Semigroup {
  fn mempty() -> Self;
}

#[allow(non_snake_case)]
mod Foldable { 
  use crate::{HKST, HKT, Monoid};
  pub trait FoldableA<'r, A: 'r>: HKST<'r, A> {
    fn fold<F>(&'r self, z: A, f: F) -> A where F: FnMut(A, &A) -> A;
    fn concat(&'r self) -> A where A: Monoid { self.fold(A::mempty(), |a,b| A::mappend(&a, b)) }
    fn find<F>(&'r self, f: F) -> Option<&A> where F: Fn(&A) -> bool;
    fn all<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    fn any<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    fn filter<F>(&'r self, f: F) -> Self::M where F: Fn(&A) -> bool;
    fn is_empty(&'r self) -> bool;
  }

  pub trait FoldableS<'r, A: 'r>: HKST<'r, A> {
    fn fold<F>(&'r self, z: A, f: F) -> A where F: Fn(&A) -> A;
    fn find<F>(&'r self, f: F) -> Option<&A> where F: Fn(&A) -> bool;
    fn all<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    fn any<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    fn filter<F>(&'r self, f: F) -> Self::M where F: Fn(&A) -> bool;
    fn is_empty(&'r self) -> bool;
  }

  pub trait FoldableB<B>: HKT<B> {
    fn fold_right<F>(&self, z: B, f: F) -> B where F: Fn(&Self::A, B) -> B;
    fn fold_left<F>(&self, z: B, f: F) -> B where F: Fn(B, &Self::A) -> B;
    fn fold_map<F>(&self, f: F) -> B where F: Fn(&Self::A) -> B, B: Monoid { self.fold_left(B::mempty(), |b, a| B::mappend(&b, &f(&a))) }
  }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

pub fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}