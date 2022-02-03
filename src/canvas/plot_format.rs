pub trait PlotFormat {
    fn into_pf(&self) -> Vec<(f32, f32)>;
}

impl PlotFormat for Vec<(f32, f32)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.clone()
    }
}

impl PlotFormat for Vec<(f64, f64)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(i32, i32)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(i64, i64)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(i16, i16)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(i8, i8)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(u8, u8)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(u16, u16)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(u32, u32)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(u64, u64)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<(usize, usize)> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter().map(|(x, y)| (*x as f32, *y as f32)).collect()
    }
}

impl PlotFormat for Vec<f32> {
    fn into_pf(&self) -> Vec<(f32, f32)> {
        self.iter()
            .enumerate()
            .map(|(i, x)| (i as f32, *x))
            .collect()
    }
}
