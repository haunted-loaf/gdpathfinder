use std::sync::mpsc::Receiver;

use godot::prelude::*;

#[derive(GodotClass)]
pub struct KDNavJob {
  base: Base<RefCounted>,
  #[var]
  path: Array<i64>,
  pub rx: Option<Receiver<Vec<i64>>>,
}

#[godot_api]
impl IRefCounted for KDNavJob {
  fn init(base: Base<RefCounted>) -> Self {
    Self {
      base,
      path: array![],
      rx: None,
    }
  }
}

#[godot_api]
impl KDNavJob {
  #[signal]
  pub fn path_found(path: Array<i64>);

  pub fn deliver_path(&mut self, path: &Array<i64>) {
    self.path = path.clone();
    self.signals().path_found().emit(path);
  }

  pub fn from_rx(rx: Receiver<Vec<i64>>) -> Gd<Self> {
    Gd::from_init_fn(|base| Self {
      base,
      path: array![],
      rx: Some(rx),
    })
  }
}
