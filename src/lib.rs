// #![warn(missing_docs)]

//! Plotting library for the Bevy game engine. To get started quickly, run a Bevy ```App```, instantiate
//! a ```Plot``` struct, and either use the
//! *  ```plot(my_data: impl Plotable)``` method for a regular graph or the
//! * ```plotm(my_data: impl Plotable)``` method for a scatter plot (or plot with markers).
//!
//! The ```my_data``` argument of either method has to implement the ```Plotable``` trait
//! (e.g. ```Vec<Vec2>```, ```Vec<(f32, f32)>```, ```Vec<f32>```, etc.).
//!
//! ```
//!  use bevy::prelude::*;
//!  use bevy_plot::*;
//!  
//!  fn main() {
//!      App::new()
//!          .add_plugins(DefaultPlugins)
//!          .add_plugin(PlotPlugin)
//!          .add_startup_system(setup)
//!          .run();
//!  }
//!  
//!  fn setup(mut commands: Commands, mut plots: ResMut<Assets<Plot>>) {
//!      commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//!  
//!      let mut plot = Plot::default();
//!  
//!      let xs = (0..30).map(|i| i as f32 / 30.0).collect::<Vec<f32>>();
//!  
//!      let ys = xs
//!          .iter()
//!          .map(|x| Vec2::new(*x, 0.5 * *x))
//!          .collect::<Vec<Vec2>>();
//!  
//!      plot.plot(ys);
//!  
//!      let plot_handle = plots.add(plot.clone());
//!      commands.spawn().insert(plot_handle);
//!  }
//!  ```
//! This library also supports plotting of explicit functions through the ```plot_func(arg: fn(f32, f32) -> 32)``` method,
//! where ```arg``` is a function that takes two arguments (x and time) and returns a ```f32```.
//!
//! For customizing the look of the graph, see the ```Opt``` enum for the available options and the ```plotopt``` and
//! ```plotopt_func``` methods.

mod plot;
pub use plot::*;

mod canvas;
pub use canvas::*;

mod markers;
pub use markers::*;

mod bezier;
pub use bezier::*;

mod segments;
pub use segments::*;

mod inputs;
mod util;
