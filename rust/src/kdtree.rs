// kd-tree for godot::Vector3, using Euclidean distance

use godot::builtin::Vector3;

pub struct Node {
  point: Vector3,
  id: i64,
  axis: usize,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

pub struct HeapEntry {
  pub distance: f64,
  pub point: Vector3,
  pub index: i64,
}

pub struct Tree {
  root: Option<Box<Node>>,
  size: usize,
}

impl Tree {
  pub fn new() -> Self {
    Self {
      root: None,
      size: 0,
    }
  }

  // insert many points into the KD-tree, balancing them
  pub fn build(&mut self, points: &[Vector3]) {
    let indices: Vec<i64> = (0..points.len()).map(|i| i as i64).collect();
    self.root = Self::build_rec(points, &indices, 0);
    self.size = points.len();
  }

  fn build_rec(points: &[Vector3], indices: &[i64], depth: usize) -> Option<Box<Node>> {
    if points.is_empty() {
      return None;
    }
    let axis = depth % 3;
    let mut sorted_points: Vec<(Vector3, i64)> = points
      .iter()
      .cloned()
      .zip(indices.iter().cloned())
      .collect();
    sorted_points.sort_unstable_by(|a, b| {
      let a_val = match axis {
        0 => a.0.x,
        1 => a.0.y,
        _ => a.0.z,
      };
      let b_val = match axis {
        0 => b.0.x,
        1 => b.0.y,
        _ => b.0.z,
      };
      a_val.partial_cmp(&b_val).unwrap()
    });
    let mid = sorted_points.len() / 2;
    let (median_point, median_index) = sorted_points[mid];
    let left_points: Vec<Vector3> = sorted_points[..mid].iter().map(|(p, _)| *p).collect();
    let left_indices: Vec<i64> = sorted_points[..mid].iter().map(|(_, i)| *i).collect();
    let right_points: Vec<Vector3> = sorted_points[mid + 1..].iter().map(|(p, _)| *p).collect();
    let right_indices: Vec<i64> = sorted_points[mid + 1..].iter().map(|(_, i)| *i).collect();
    Some(Box::new(Node {
      point: median_point,
      id: median_index,
      axis,
      left: Self::build_rec(&left_points, &left_indices, depth + 1),
      right: Self::build_rec(&right_points, &right_indices, depth + 1),
    }))
  }

  pub fn nearest(&self, target: Vector3) -> Option<HeapEntry> {
    self.root.as_ref()?;
    let mut best = HeapEntry {
      distance: f64::INFINITY,
      point: Vector3::ZERO,
      index: -1,
    };
    Self::nearest_rec(&self.root, &target, &mut best);
    Some(best)
  }

  fn nearest_rec(node: &Option<Box<Node>>, target: &Vector3, best: &mut HeapEntry) {
    if node.is_none() {
      return;
    }
    let node = node.as_ref().unwrap();
    let d2 = target.distance_squared_to(node.point);
    if d2 < best.distance {
      best.distance = d2;
      best.point = node.point;
      best.index = node.id;
    }

    let axis = node.axis;
    let delta = match axis {
      0 => target.x - node.point.x,
      1 => target.y - node.point.y,
      _ => target.z - node.point.z,
    };
    let (near, far) = if delta < 0.0 {
      (&node.left, &node.right)
    } else {
      (&node.right, &node.left)
    };

    Self::nearest_rec(near, target, best);

    if (delta * delta) < best.distance {
      Self::nearest_rec(far, target, best);
    }
  }

  // pub fn nearest_k(&self, target: Vector3, k: usize) -> Vec<HeapEntry> {
  //   if self.root.is_none() || k == 0 {
  //     return vec![];
  //   }
  //   let mut heap: Vec<HeapEntry> = vec![];
  //   Self::nearest_k_rec(&self.root, &target, k, &mut heap);
  //   heap.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
  //   heap
  // }

  // fn nearest_k_rec(
  //   node: &Option<Box<Node>>,
  //   target: &Vector3,
  //   k: usize,
  //   heap: &mut Vec<HeapEntry>,
  // ) {
  //   if node.is_none() {
  //     return;
  //   }
  //   let node = node.as_ref().unwrap();
  //   let d2 = target.distance_squared_to(node.point);
  //   Self::heap_push_candidate(heap, d2, node.point, node.id, k);

  //   let axis = node.axis;
  //   let delta = match axis {
  //     0 => target.x - node.point.x,
  //     1 => target.y - node.point.y,
  //     _ => target.z - node.point.z,
  //   };
  //   let (near, far) = if delta < 0.0 {
  //     (&node.left, &node.right)
  //   } else {
  //     (&node.right, &node.left)
  //   };

  //   Self::nearest_k_rec(near, target, k, heap);

  //   let worst_d2 = if heap.len() < k {
  //     f64::INFINITY
  //   } else {
  //     heap.iter().map(|e| e.distance).fold(f64::MIN, f64::max)
  //   };
  //   if (delta * delta) < worst_d2 {
  //     Self::nearest_k_rec(far, target, k, heap);
  //   }
  // }

  pub fn radius_search(&self, target: Vector3, r: f64) -> Vec<i64> {
    if self.root.is_none() || r < 0.0 {
      return vec![];
    }
    let r2 = r * r;
    let mut out: Vec<HeapEntry> = vec![];
    Self::radius_search_rec(&self.root, &target, r2, &mut out);
    out.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    out.iter().map(|e| e.index).collect()
  }

  fn radius_search_rec(
    node: &Option<Box<Node>>,
    target: &Vector3,
    r2: f64,
    out: &mut Vec<HeapEntry>,
  ) {
    if node.is_none() {
      return;
    }
    let node = node.as_ref().unwrap();
    let d2 = target.distance_squared_to(node.point);
    if d2 <= r2 {
      out.push(HeapEntry {
        distance: d2,
        point: node.point,
        index: node.id,
      });
    }

    let axis = node.axis;
    let delta = match axis {
      0 => target.x - node.point.x,
      1 => target.y - node.point.y,
      _ => target.z - node.point.z,
    };
    let (near, far) = if delta < 0.0 {
      (&node.left, &node.right)
    } else {
      (&node.right, &node.left)
    };

    Self::radius_search_rec(near, target, r2, out);
    if (delta * delta) <= r2 {
      Self::radius_search_rec(far, target, r2, out);
    }
  }

  // fn heap_push_candidate(heap: &mut Vec<HeapEntry>, d2: f64, p: Vector3, i: i64, k: usize) {
  //   if heap.len() < k {
  //     heap.push(HeapEntry {
  //       distance: d2,
  //       point: p,
  //       index: i,
  //     });
  //     return;
  //   }
  //   let max_i = heap
  //     .iter()
  //     .enumerate()
  //     .max_by(|a, b| a.1.distance.partial_cmp(&b.1.distance).unwrap())
  //     .map(|(idx, _)| idx)
  //     .unwrap();
  //   if d2 < heap[max_i].distance {
  //     heap[max_i] = HeapEntry {
  //       distance: d2,
  //       point: p,
  //       index: i,
  //     };
  //   }
  // }
}
