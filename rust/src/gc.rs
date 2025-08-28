use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct Gc<T>(Arc<UnsafeCell<T>>);

unsafe impl<T> Sync for Gc<T> {}
unsafe impl<T> Send for Gc<T> {}

impl<T> Gc<T> {
  pub fn new(t: T) -> Self {
    Self(Arc::new(UnsafeCell::new(t)))
  }
  #[allow(clippy::mut_from_ref)]
  pub fn get(&self) -> &mut T {
    unsafe { &mut *self.0.get() }
  }
}

impl<T> Clone for Gc<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> std::ops::Deref for Gc<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0.get() }
  }
}

impl<T> std::ops::DerefMut for Gc<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0.get() }
  }
}

impl From<Gc<std::vec::Vec<i64>>> for std::vec::Vec<i64> {
  fn from(value: Gc<std::vec::Vec<i64>>) -> Self {
    value.deref().clone()
  }
}
