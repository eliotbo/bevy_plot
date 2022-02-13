
![logo](bevy_plot_log.png)

Plotting library for the Bevy game engine with a focus on esthetics and interactivity. It can handle both data points (see the "minimal", "markers", and "bevy" examples) and explicit functions (see the "func", "animate" and "runtime_setter" examples). Explicit functions are rendered using quadratic Bezier interpolation, thus smoothing out the curves.

![animate](stiched.gif)

## How to get started

Add "bevy_plot" to the dependencies list in the Cargo.toml file of your project, and have a look at the [examples](https://github.com/eliotbo/bevy_plot/tree/main/examples) to see how to add the PlotPlugin, import and use the Plot asset.

## TODO

- reduce API boilerplate
- interactive markers
- compatibility with 3d camera
- optimization
