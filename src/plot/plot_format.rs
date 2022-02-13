use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct PlotFormat {
    pub data: Vec<Vec2>,
}

pub trait Plotable {
    fn into_plot_format(&self) -> PlotFormat;
}

impl Plotable for Vec<Vec2> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat { data: self.clone() }
    }
}

impl Plotable for Vec<(f64, f64)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(f32, f32)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(i32, i32)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(i64, i64)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(i16, i16)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(i8, i8)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(u8, u8)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(u16, u16)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(u32, u32)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(u64, u64)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<(usize, usize)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
        }
    }
}

impl Plotable for Vec<f32> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .enumerate()
                .map(|(i, x)| Vec2::new(i as f32, *x))
                .collect(),
        }
    }
}

impl Plotable for Vec<f64> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .enumerate()
                .map(|(i, x)| Vec2::new(i as f32, *x as f32))
                .collect(),
        }
    }
}
