
![logo](bevy_plot3.png)

Plotting library for the Bevy game engine with a focus on esthetics and interactivity. It can handle both data points (see the "minimal", "markers", and "bevy" examples) and explicit functions (see the "func", "animate" and "runtime_setter" examples). Explicit functions are rendered using quadratic Bezier interpolation, thus smoothing out the curves. 

Here is a link to the [docs](https://docs.rs/bevy_plot/0.1.3/bevy_plot/).

![animate](stiched.gif)

## How to get started

Add "bevy_plot" to the dependencies list in the Cargo.toml file of your project, add a font to your assets, and have a look at the [examples](https://github.com/eliotbo/bevy_plot/tree/main/examples) to see how to add the PlotPlugin, import and use the Plot asset.

## TODO

- reduce API boilerplate
- interactive markers
- compatibility with 3d camera
- optimization
