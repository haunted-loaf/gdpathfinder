use std::sync::{Arc, Mutex, mpsc};
use threadpool::ThreadPool;

use crate::job::KDNavJob;
use crate::kdnavi::KDNavI;

use godot::{prelude::*};

#[derive(GodotClass)]
#[class(base=Node, tool)]
struct KDNav {
  i: Arc<Mutex<KDNavI>>,
  base: Base<Node>,
  pool: ThreadPool,
  jobs: Vec<Gd<KDNavJob>>,
}

#[godot_api]
impl INode for KDNav {
  fn init(base: Base<Node>) -> Self {
    Self {
      i: Arc::new(Mutex::new(KDNavI::new())),
      base,
      pool: ThreadPool::new(1),
      jobs: vec![],
    }
  }

  fn process(&mut self, _delta: f64) {
    self.jobs.retain_mut(|job| {
      let mut job = job.bind_mut();
      if let Some(rx) = &job.rx
        && let Ok(path) = rx.try_recv()
      {
        let path = path.into_iter().collect::<Array<_>>();
        job.deliver_path(&path);
        return false;
      }
      true
    })
  }
}

#[godot_api]
impl KDNav {
  #[signal]
  fn path_found(path: Array<i64>);

  #[func]
  fn path(&mut self, start: i64, end: i64, maxdist: f64) -> Gd<KDNavJob> {
    let (tx, rx) = mpsc::channel();
    let job = KDNavJob::from_rx(rx);
    let i = self.i.clone();
    self.pool.execute(move || {
      let path = i.lock().unwrap().path(start, end, maxdist);
      let path: Vec<i64> = path.into_iter().collect();
      tx.send(path).unwrap();
    });
    self.jobs.push(job.clone());
    job
  }

  #[func]
  fn nearest(&self, v: Vector3) -> i64 {
    self.i.lock().unwrap().nearest(v)
  }

  #[func]
  fn neighbors(&mut self, id: i64, maxdist: f64) -> Vec<i64> {
    self.i.lock().unwrap().neighbors(id, maxdist).to_vec()
  }

  #[func]
  fn build(&mut self, points: Vec<Vector3>) {
    self.i.lock().unwrap().build(points)
  }

  #[func]
  fn clear(&mut self) {
    self.jobs.clear();
    self.i.lock().unwrap().clear();
  }
}
