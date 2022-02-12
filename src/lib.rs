// #![warn(missing_docs)]

pub mod canvas;
pub use canvas::*;

mod markers;
pub use markers::*;

mod inputs;
// use inputs::*;

mod util;

mod bezier;
pub use bezier::*;

mod plot;
pub use plot::*;

mod segments;
pub use segments::*;
