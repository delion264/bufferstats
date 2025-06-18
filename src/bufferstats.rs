pub struct BufferStats {
    pub average: f64,
    pub mean_buffer: Vec<f64>,
    pub stdev_buffer: Vec<f64>,
    pub variance: f64,
    pub stdev: Option<f64>,
}

impl BufferStats {
    pub fn new() -> BufferStats {
        BufferStats {
            average: 0.,
            mean_buffer: Vec::new(),  // Moving average value at each index
            stdev_buffer: Vec::new(), // Standard deviation over the moving average window
            variance: 0.,
            stdev: None,
        }
    }

    // Window size = 0 => calculate mean of whole buffer
    // Window size >=2 => moving average
    pub fn mean(&mut self, data: &[f64], window_size: usize) {
        assert!(
            window_size != 1,
            "Window size must be >= 2 for a moving average, {} provided",
            window_size
        );
        assert!(
            window_size <= data.len(),
            "Window size must be < length of data buffer"
        );
        self.mean_buffer = vec![0.; data.len()];
        self.stdev_buffer = vec![0.; data.len()];

        if window_size == 0 {
            let sum = data.iter().sum::<f64>() as f64;
            let count = data.len();
            self.average = sum / count as f64;
            self.mean_buffer.fill(self.average);
        } else if window_size >= 2 {
            for idx in 0..data.len() {
                if idx < window_size {
                    self.mean_buffer[idx] =
                        data[0..window_size].iter().sum::<f64>() as f64 / window_size as f64;
                } else {
                    self.mean_buffer[idx] = self.mean_buffer[idx - 1]
                        + (data[idx] - data[idx - window_size]) / window_size as f64;
                }
            }
        }
    }

    // pub fn std_deviation(&mut self, data: &[f64]) {
    //     self.mean(data, 0);
    //     self.stdev = match (self.average, data.len()) {
    //         (Some(data_mean), count) if count > 0 => {
    //             self.variance = data
    //                 .iter()
    //                 .map(|value| {
    //                     let diff = data_mean - (*value as f64);

    //                     diff * diff
    //                 })
    //                 .sum::<f64>()
    //                 / count as f64;

    //             Some(self.variance.sqrt())
    //         }
    //         _ => None,
    //     };
    // }

    pub fn std_deviation(&mut self, data: &[f64]) {
        self.variance = 0.;
        for i in 0..data.len() {
            let diff = self.mean_buffer[i] - data[i];
            self.variance += diff * diff;
        }
        self.stdev = Some(self.variance.sqrt() / data.len() as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_average() {
        let mut window = BufferStats::new();
        window.mean(
            &[
                12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
            ],
            0,
        );
        assert_eq!(window.average, 27.8125);
    }
    #[test]
    fn moving_average_two() {
        let mut window = BufferStats::new();
        window.mean(
            &[
                12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
            ],
            2,
        );
        assert_eq!(
            window.mean_buffer,
            [8.5, 8.5, 7., 7.5, 19., 23.5, 11.5, 4.5, 25.5, 63., 113., 107.5, 34., 2., 2., 6.]
        )
    }
    #[test]
    fn moving_average_four() {
        let mut window = BufferStats::new();
        window.mean(
            &[
                12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
            ],
            4,
        );
        assert_eq!(
            window.mean_buffer,
            [
                8., 8., 8., 8., 13., 15.5, 15.25, 14., 18.5, 33.75, 69.25, 85.25, 73.5, 54.75, 18.,
                4.
            ]
        )
    }
    #[test]
    #[should_panic]
    fn invalid_window_size() {
        let mut window = BufferStats::new();
        window.mean(
            &[
                12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
            ],
            1,
        );
    }
    #[test]
    #[should_panic]
    fn window_size_too_large() {
        let mut window = BufferStats::new();
        window.mean(
            &[
                12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
            ],
            17,
        );
    }
}
