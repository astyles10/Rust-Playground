pub trait Comparable<T> {
  fn lowest(&self) -> T;
  fn highest(&self) -> T;
}

pub struct Comparer<'a, T> {
  first: &'a [T],
  second: &'a [T]
}

impl Comparable<u8> for Comparer<'_, u8> {
  fn lowest(&self) -> u8 {
    let lowest: Option<&u8> = self.first.first();

    0
  }
  fn highest(&self) -> u8 {
    0
  }
}

