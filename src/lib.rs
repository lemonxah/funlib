//! Functional Libary for Rust
//! 
//! This is still a work in progess..
//!  
//! Functor
//! 
//! ```
//! use funlib::Functor;
//! 
//! let s = Some(3).fmap(|&a| a * 2); // this will produce Some(6)
//! 
//! ```
//! 
//! Monad 
//! 
//! ```
//! use funlib::Monad;
//! 
//! let s = Some(3).bind(|&a| Some(a * 2)); // this will produce Some(6)
//! 
//! ```
//! 
//! 

#![deny(missing_docs)]
#[macro_use] pub mod macros;
pub mod functor;
pub mod applicative;
pub mod monad;
pub mod semigroup;
pub mod monoid;
pub mod foldable;
use std::rc::Rc;

/// Higher Kinded Type helper for M<A> -> M<B>
pub trait HKT<B> {
  /// Current Type
  type A;
  /// Type M<B>
  type M;
}
/// Higher kinded Type helper for M<A> -> M<A>
pub trait HKST<'a, B> {
  /// Current Type
  type A;
  /// Type M<A>
  type M;
}

derive_hkt!(Vec);
derive_hkt!(Option);
derive_hkt!(Box);
derive_hkt!(Rc);

/// Functor type class
pub trait Functor<B>: HKT<B> {
  /// Functor map
  /// # Examples
  /// ```
  /// use funlib::Functor;
  /// let n = Some(1).fmap(|i| i * 4);
  /// assert_eq!(Some(4), n);
  /// ```
  fn fmap<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> B;
}

/// Applicative type class
pub trait Applicative<B>: Functor<B> {
  /// Lift values into the context of the Functor
  /// 
  /// # Examples
  /// ```
  /// use funlib::Applicative;
  /// let s1 = Option::<i8>::pure_(10);
  /// let s2 = Option::pure_("hi");
  /// let v = Vec::pure_(1);
  /// ```
  fn pure_(value: B) -> Self::M where Self: HKT<B, A=B>;
  /// Apply function is almost the same as Functor map. but the function isn't A => B but A<F => B>
  /// 
  /// # Examples
  /// ```
  /// use funlib::Applicative;
  /// fn double(i: &i32) -> i32 { i * 2  }
  /// let f: &dyn Fn(&i32) -> i32 = &|x| x * 2;
  /// assert_eq!(Some(4), Some(2).ap(Some(f)));
  /// assert_eq!(Some(4), Some(2).ap(Some(&double)));
  /// ```
  fn ap<F>(&self, f: <Self as HKT<F>>::M) -> <Self as HKT<B>>::M
      where F: Fn(&<Self as HKT<B>>::A) -> B, Self:HKT<F>;
}

/// Monad type class
pub trait Monad<B>: Applicative<B> {
  /// Bind works like map but it flattens nested structures
  /// 
  /// # Examples
  /// ```
  /// use funlib::Applicative;
  /// use funlib::Monad;
  /// fn over5(i: &i32) -> Option<i32> { if *i > 5 { Some(*i) } else { None }}
  /// let a = Some(4).bind(over5);
  /// let b = Some(6).bind(over5);
  /// assert_eq!(None, a);
  /// assert_eq!(Some(6), b);
  /// ```
  fn bind<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> Self::M;
}

/// Semigroup type class
pub trait Semigroup: Clone {
  /// combine 2 of the same type 
  /// 
  /// # Examples
  /// 
  /// ```
  /// use funlib::Semigroup;
  /// assert_eq!(4i32, 1i32.mappend(&3i32));
  /// assert_eq!(Some(4i32), Some(1i32).mappend(&Some(3i32)));
  /// ```
  fn mappend(&self, other: &Self) -> Self;
}

/// Monoid type class extends the Semigroup and adds an empty function for the type
pub trait Monoid: Semigroup {
  /// empty function same as Default
  /// 
  /// # Examples
  /// 
  /// ```
  /// use funlib::{Monoid, Semigroup, Foldable::*};
  /// let sum = vec![1i32,2i32,3i32,4i32].fold(i32::mempty(), |b,a| i32::mappend(&b, a));
  /// assert_eq!(10i32, sum);
  /// assert_eq!(None::<i32>, Option::<i32>::mempty());
  /// ```
  fn mempty() -> Self;
}

/// Foldable mod containing the foldable type classes
#[allow(non_snake_case)]
pub mod Foldable { 
  use crate::{HKST, HKT, Monoid};
  /// FoladableA is for endo type functions
  pub trait FoldableA<'r, A: 'r>: HKST<'r, A> {
    /// Reduces the values of the Foldable into a single value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// let sum = v.fold(0, |b, a| a + b);
    /// assert_eq!(10, sum);
    /// ```
    fn fold<F>(&'r self, z: A, f: F) -> A where F: FnMut(A, &A) -> A;
    /// Using a Monoid reduce the values in the Foldable to a single value
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// let sum = v.concat();
    /// assert_eq!(10, sum);
    /// ```
    fn concat(&'r self) -> A where A: Monoid { self.fold(A::mempty(), |a,b| A::mappend(&a, b)) }
    /// Find a value in the foldable, returns an Option<&_>
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// let s = v.find(|&a| a == 2);
    /// let n = v.find(|&a| a == 5);
    /// assert_eq!(Some(&2), s);
    /// assert_eq!(None, n);
    /// ```
    fn find<F>(&'r self, f: F) -> Option<&A> where F: Fn(&A) -> bool;
    /// Check if all values in the foldable returns true for function f
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// assert_eq!(true, v.all(|&a| a < 5));
    /// assert_eq!(false, v.all(|&a| a < 4));
    /// ```
    fn all<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    /// Check if any valu ein the foldable returns true for function f
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// assert_eq!(true, v.any(|&a| a == 4));
    /// assert_eq!(false, v.any(|&a| a == 5));
    /// ```
    fn any<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    /// Filters the foldable for values that meet the predicate
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// assert_eq!(vec![&1,&2], v.filter(|&a| a < 3));
    /// ```
    fn filter<F>(&'r self, f: F) -> Self::M where F: Fn(&A) -> bool;
    /// Checks if the foldable is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// let v2: Vec<i32> = vec![];
    /// assert_eq!(false, v.is_empty());
    /// assert_eq!(true, v2.is_empty());
    /// ```
    fn is_empty(&'r self) -> bool;
    /// Checks if the foldable is non empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// let v = vec![1,2,3,4];
    /// let v2: Vec<i32> = vec![];
    /// assert_eq!(true, v.non_empty());
    /// assert_eq!(false, v2.non_empty());
    /// ```
    fn non_empty(&'r self) -> bool { !self.is_empty() }
  }

  /// FoldableS is for Foldables that is not a list of some kind, ex. Option
  pub trait FoldableS<'r, A: 'r>: HKST<'r, A> {
    /// Reduces the values of the Foldable into a single value
    fn fold<F>(&'r self, z: A, f: F) -> A where F: Fn(&A) -> A;
    /// Find a value in the foldable, returns an Option<&_>
    fn find<F>(&'r self, f: F) -> Option<&A> where F: Fn(&A) -> bool;
    /// Check if all values in the foldable returns true for function f
    fn all<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    /// Check if any valu ein the foldable returns true for function f
    fn any<F>(&'r self, f: F) -> bool where F: Fn(&A) -> bool;
    /// Filters the foldable for values that meet the predicate
    fn filter<F>(&'r self, f: F) -> Self::M where F: Fn(&A) -> bool;
    /// Checks if the foldable is empty
    fn is_empty(&'r self) -> bool;
    /// Checks if the foldable is non empty.
    fn non_empty(&'r self) -> bool { !self.is_empty() }
  }

  /// FoladableB is for Hinger Kinded Types where M<A> -> B / M<B>
  pub trait FoldableB<B>: HKT<B> {
    /// Reduces the values of the Foldable into a single value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// #[derive(Debug, PartialEq)]
    /// struct Count(i32);
    /// let v = vec![1,2,3,4];
    /// let sum = v.fold_right(Count(0), |a, b| Count(a + b.0));
    /// assert_eq!(Count(10), sum);
    /// ```
    fn fold_right<F>(&self, z: B, f: F) -> B where F: Fn(&Self::A, B) -> B;
    /// Reduces the values of the Foldable into a single value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use funlib::Foldable::*;
    /// #[derive(Debug, PartialEq)]
    /// struct Count(i32);
    /// let v = vec![1,2,3,4];
    /// let sum: Count = v.fold_left(Count(0), |b, a| Count(a + b.0));
    /// assert_eq!(Count(10), sum);
    /// ```
    fn fold_left<F>(&self, z: B, f: F) -> B where F: Fn(B, &Self::A) -> B;
    /// Using a Monoid and a function to transform the Foldable values form A -> b to reduce the values in the Foldable to a single value of B
    fn fold_map<F>(&self, f: F) -> B where F: Fn(&Self::A) -> B, B: Monoid { self.fold_left(B::mempty(), |b, a| B::mappend(&b, &f(&a))) }
  }
}
