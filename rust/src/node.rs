use crate::O64;
use crate::gc::Gc;

pub struct InnerNode {
  pub id: i64,
  pub g: O64,
  pub up: Option<Node>,
}

pub struct Node(Gc<InnerNode>);

impl Node {
  pub fn new(id: impl Into<i64>, g: impl Into<O64>, up: Option<Node>) -> Self {
    Self(Gc::new(InnerNode {
      id: id.into(),
      g: g.into(),
      up: up.clone(),
    }))
  }
  pub fn id(id: impl Into<i64>) -> Self {
    Self(Gc::new(InnerNode {
      id: id.into(),
      g: 0.into(),
      up: None,
    }))
  }
}

impl Clone for Node {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl std::ops::Deref for Node {
  type Target = InnerNode;
  fn deref(&self) -> &Self::Target {
    &*self.0.get()
  }
}

impl std::ops::DerefMut for Node {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut *self.0.get()
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for Node {}

impl std::hash::Hash for Node {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.id.hash(state)
  }
}
