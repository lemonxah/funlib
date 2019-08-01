pub mod functor;
pub mod applicative;
pub mod monad;
pub mod semigroup;
pub mod monoid;
use std::rc::Rc;

pub trait HKT<B> {
  type A; // Current type
  type M; // Type A swapped with B
}

macro_rules! derive_hkt {
  ($t:ident) => {
    impl<B, C> HKT<C> for $t<B> {
      type A = B;
      type M = $t<C>;
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
  fn mappend(self: &Self, other: &Self) -> Self;
}

pub trait Monoid: Semigroup {
  fn mempty() -> Self;
}

#[allow(unused_macros)]
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