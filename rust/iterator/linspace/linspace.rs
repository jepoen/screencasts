// mod linspace

pub struct LinSpace {
    current: f64,
    stop: f64,
    delta: f64,
}

impl LinSpace {
    pub fn new(start: f64, stop: f64, n_steps: usize) -> Option<LinSpace> {
        if start >= stop {
            None
        } else {
            let delta = (stop - start) / n_steps as f64;
            Some(Self {
                current: start,
                stop: stop + delta * 0.5,
                delta,
            })
        }
    }
}

impl Iterator for LinSpace {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.stop {
            None
        } else {
            let res = self.current;
            self.current += self.delta;
            Some(res)
        }
    }
}
