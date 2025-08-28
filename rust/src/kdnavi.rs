use crate::O64;
use crate::gc::Gc;
use crate::node::Node;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use godot::prelude::*;
use priority_queue::PriorityQueue;

pub struct KDNavI {
  pub points: Vec<Vector3>,
  pub tree: crate::kdtree::Tree,
  pub neighbor_cache: Gc<HashMap<i64, Vec<i64>>>,
}

impl KDNavI {
  pub fn new() -> Self {
    Self {
      points: vec![],
      tree: crate::kdtree::Tree::new(),
      neighbor_cache: Gc::new(HashMap::new()),
    }
  }

  pub fn clear(&mut self) {
    self.points.clear();
    self.tree = crate::kdtree::Tree::new();
    self.neighbor_cache.get().clear();
  }

  pub fn build(&mut self, points: Vec<Vector3>) {
    self.points = points.clone();
    self.neighbor_cache.get().clear();
    self.tree.build(&points);
  }

  pub fn path(&mut self, start: i64, end: i64, maxdist: f64) -> Vec<i64> {
    let mut open = PriorityQueue::new();
    let mut closed: HashSet<i64> = HashSet::new();
    open.push(Node::new(start, 0, None), Reverse(O64::from(0.0)));
    while !open.is_empty() {
      let (current, _) = open.pop().unwrap();
      closed.insert(current.id);
      if current.id == end {
        let mut path: Vec<i64> = vec![];
        let mut node = current;
        path.push(node.id);
        while let Some(up) = node.up.clone() {
          path.push(up.id);
          node = up;
        }
        path.reverse();
        return path;
      }
      let neighbors = self.neighbors(current.id, maxdist);
      for neighbor in neighbors {
        if closed.contains(neighbor) {
          continue;
        }
        let g =
          current.g + self.points[current.id as usize].distance_to(self.points[*neighbor as usize]);
        let h = self.points[*neighbor as usize].distance_to(self.points[end as usize]);
        let f = g + h;
        if let Some((neighbor, _)) = open.get_mut(&Node::id(*neighbor)) {
          if g > neighbor.g {
            continue;
          }
          neighbor.up = Some(current.clone());
        }
        open.push(
          Node::new(*neighbor, g, Some(current.clone())),
          Reverse(O64::from(f)),
        );
      }
    }
    vec![]
  }

  pub fn nearest(&self, v: Vector3) -> i64 {
    match self.tree.nearest(v) {
      Some(node) => node.index,
      None => -1,
    }
  }

  pub fn neighbors(&self, id: i64, maxdist: f64) -> &[i64] {
    if self.neighbor_cache.contains_key(&id) {
      return self.neighbor_cache.get().get(&id).unwrap();
    }
    let point = self.points[id as usize];
    let results = self.tree.radius_search(point, maxdist).clone();
    self.neighbor_cache.get().insert(id, results);
    self.neighbor_cache.get().get(&id).unwrap().as_ref()
  }
}
