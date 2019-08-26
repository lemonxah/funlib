use crate::{FoldableB, FoldableA};

impl <A, B> FoldableB<B> for Vec<A> {
  fn fold_right<F>(&self, z: B, f: F) -> B where F: Fn(&Self::A, B) -> B {
    let mut r: B = z;
    for x in self.iter().rev() {
      r = f(x, r);
    }
    r
  }
  fn fold_left<F>(&self, z: B, f: F) -> B where F: Fn(B, &Self::A) -> B {
    self.iter().fold(z, f)
  }
}

impl <'r,A: 'r> FoldableA<'r, A> for Vec<A> {
  fn fold<F>(self: &'r Self, z: A, f: F) -> A where F: FnMut(A, &A) -> A {
    self.iter().fold(z, f)
  }
  fn all<F>(&self, f: F) -> bool where F: Fn(&A) -> bool {
    self.iter().all(f)
  }
  fn any<F>(&self, f: F) -> bool where F: Fn(&A) -> bool {
    self.iter().any(f)
  }
  fn filter<F>(self: &'r Self, f: F) -> Self::M where F: Fn(&A) -> bool {
    self.iter().filter(|a| f(a)).collect()
  }
  fn find<F>(self: &'r Self, f: F) -> Option<&A> where F: Fn(&A) -> bool {
    self.iter().find(|a| f(a))
  }
  fn is_empty(&self) -> bool { self.is_empty() }
}

#[cfg(test)]
mod test {
  use crate::{FoldableA, FoldableB};

  #[test]
  fn vec_filter() {
    let v1 = vec![1,2,3,4];
    assert_eq!(vec![&2,&4], v1.filter(|&a| a%2==0))
  }

  #[test]
  fn vec_filter_then_fold_left() {
    let v1 = vec![1,2,3,4];
    assert_eq!(6, v1.filter(|&a| a%2==0).fold_left(0, |b,&a| a + b));
  }

  #[test]
  fn vec_filter_then_fold_right() {
    let v1 = vec![1,2,3,4];
    assert_eq!(6, v1.filter(|&a| a%2==0).fold_right(0, |&a,b| a + b));
  }

}