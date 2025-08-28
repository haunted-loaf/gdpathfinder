use godot::prelude::*;
use ordered_float::OrderedFloat;

struct PointNavExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PointNavExtension {
  
}

pub type O64 = OrderedFloat<f64>;

mod kdnav;
mod kdnavi;
mod kdtree;
mod node;
mod gc;
mod job;
