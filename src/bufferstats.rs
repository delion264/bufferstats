pub struct BufferStats {
    pub median: Option<f64>,
    pub variance: f64,
    pub stdev: Option<f64>,
}

impl BufferStats {
    pub fn new() -> BufferStats {
        BufferStats {
            median: None,
            variance: 0.,
            stdev: None,
        }
    }
    pub fn mean(&mut self, data: &[f64]) {
        let sum = data.iter().sum::<f64>() as f64;
        let count = data.len();

        match count {
            positive if positive > 0 => self.median = Some(sum / count as f64),
            _ => self.median = None,
        }
    }

    pub fn std_deviation(&mut self, data: &[f64]) {
        self.mean(data);
        self.stdev = match (self.median, data.len()) {
            (Some(data_mean), count) if count > 0 => {
                self.variance = data
                    .iter()
                    .map(|value| {
                        let diff = data_mean - (*value as f64);

                        diff * diff
                    })
                    .sum::<f64>()
                    / count as f64;

                Some(self.variance.sqrt())
            }
            _ => None,
        };
    }
}
