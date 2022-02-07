use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum PlotType {
    Marker,
    Segment,
    Bezier,
}

#[derive(Debug, Clone)]
pub struct PlotFormat {
    pub data: Vec<Vec2>,
    pub ptype: PlotType,
    pub maybe_func: Option<fn(f32) -> f32>,
}

// where
//     F: Fn(f32) -> f32,

pub trait Plotable {
    fn into_plot_format(&self) -> PlotFormat;
}

impl Plotable for Vec<Vec2> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self.clone(),
            ptype: PlotType::Marker,
            maybe_func: None,
        }
    }
}

impl Plotable for Vec<(f64, f64)> {
    fn into_plot_format(&self) -> PlotFormat {
        PlotFormat {
            data: self
                .iter()
                .map(|(x, y)| Vec2::new(*x as f32, *y as f32))
                .collect(),
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
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
            ptype: PlotType::Marker,
            maybe_func: None,
        }
    }
}

// impl Plotable for fn(f32) -> f32 {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: vec![],
//             ptype: PlotType::Bezier,
//             maybe_func: Some(*self),
//         }
//     }
// }

// impl Plotable for Box<dyn Fn(f32) -> f32> {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: vec![],
//             ptype: PlotType::Bezier,
//             maybe_func: Some(*self),
//         }
//     }
// }

// impl<F> Plotable for F
// where
//     F: Fn(f32) -> f32,
// {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: vec![],
//             ptype: PlotType::Bezier,
//             maybe_func: Some(*self),
//         }
//     }
// }

// impl Plotable for dyn Fn(f32) -> f32 {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: vec![],
//             ptype: PlotType::Bezier,
//             maybe_func: Some(*self),
//         }
//     }
// }

// impl Plotable for (Vec<f32>, dyn Fn(f32) -> f32) {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: self.0.iter().map(|x| Vec2::new(*x, self.1(*x))).collect(),
//             ptype: PlotType::Marker, maybe_func: None,
//         }
//     }
// }

// impl Plotable for (Vec<f64>, dyn Fn(f32) -> f32) {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: self
//                 .0
//                 .iter()
//                 .map(|x| Vec2::new(*x as f32, self.1(*x as f32)))
//                 .collect(),
//             ptype: PlotType::Marker, maybe_func: None,
//         }
//     }
// }

// impl Plotable for (Vec<f32>, dyn Fn(f64) -> f64) {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: self
//                 .0
//                 .iter()
//                 .map(|x| Vec2::new(*x, self.1(*x as f64) as f32))
//                 .collect(),
//             ptype: PlotType::Marker, maybe_func: None,
//         }
//     }
// }

// impl Plotable for (Vec<f64>, dyn Fn(f64) -> f64) {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: self
//                 .0
//                 .iter()
//                 .map(|x| Vec2::new(*x as f32, self.1(*x) as f32))
//                 .collect(),
//             ptype: PlotType::Marker, maybe_func: None,
//         }
//     }
// }

// // TODO: all the cases for ints and uints, usize...
// impl Plotable for (Vec<f64>, dyn Fn(u32) -> f32) {
//     fn into_plot_format(&self) -> PlotFormat {
//         PlotFormat {
//             data: self
//                 .0
//                 .iter()
//                 .map(|x| Vec2::new(*x as f32, self.1(*x as u32) as f32))
//                 .collect(),
//             ptype: PlotType::Marker, maybe_func: None,
//         }
//     }
// }
