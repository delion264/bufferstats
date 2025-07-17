use ringbuffer::{AllocRingBuffer, RingBuffer};

pub struct BufferStats {
    pub buffer_size: usize,
    pub window_size: usize,
    pub total_avg: f64,
    pub total_variance: f64,
    pub total_stdev: f64,
    pub moving_avg: AllocRingBuffer<f64>,
    pub moving_variance: AllocRingBuffer<f64>,
}

impl BufferStats {
    pub fn new(buffer_len: usize, window_len: usize) -> BufferStats {
        BufferStats {
            buffer_size: buffer_len,
            window_size: window_len,
            total_avg: 0.,
            total_variance: 0.,
            total_stdev: 0.,
            moving_avg: AllocRingBuffer::new(buffer_len),
            moving_variance: AllocRingBuffer::new(buffer_len), // Standard deviation over the moving average window
        }
    }

    // Rolling buffer initialisation methods
    pub fn init_moving_avg(&mut self, data: &[f64]) {
        let mut init_moving_avg_value = 0.;
        for idx in 0..self.window_size - 1 {
            init_moving_avg_value += data[idx];
        }
        init_moving_avg_value /= self.window_size as f64;

        for idx in 0..self.buffer_size {
            if idx < self.window_size {
                self.moving_avg.push(init_moving_avg_value);
            } else {
                self.update_moving_avg(&data);
            }
        }
    }

    pub fn init_moving_variance(&mut self, data: &[f64]) {
        let mut variance = 0.;
        for idx in 0..self.buffer_size {
            if idx < self.window_size {
                let diff = self.moving_avg[idx] - data[idx];
                variance += diff * diff;
                self.moving_variance
                    .push(variance / self.window_size as f64);
            } else {
                self.update_moving_variance(&data);
            }
        }
    }

    // Rolling buffer update methods
    pub fn update_moving_avg(&mut self, data: &[f64]) {
        let oldest_data = data[0];
        let newest_data = data[data.len() - 1];
        let newest_avg = self.moving_avg.back().unwrap();
        self.moving_avg
            .push(newest_avg + (newest_data - oldest_data) / self.window_size as f64)
    }

    pub fn update_moving_variance(&mut self, data: &[f64]) {
        let oldest_data = data[0];
        let newest_data = data[data.len() - 1];
        let oldest_avg = self
            .moving_avg
            .get_signed(-(self.window_size as isize))
            .unwrap();
        let newest_avg = self.moving_avg.back().unwrap();
        let oldest_diff = (oldest_data - oldest_avg).powi(2);
        let newest_diff = (newest_data - newest_avg).powi(2);
        let newest_variance = self.moving_variance.back().unwrap();
        self.moving_variance
            .push(newest_variance + (newest_diff - oldest_diff) / self.window_size as f64)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn total_average() {
//         let mut window = BufferStats::new();
//         window.find_moving_avg(
//             &[
//                 12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
//             ],
//             0,
//         );
//         assert_eq!(window.stationary_avg, 27.8125);
//     }
//     #[test]
//     fn moving_average_two() {
//         let mut window = BufferStats::new();
//         window.find_moving_avg(
//             &[
//                 12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
//             ],
//             2,
//         );
//         assert_eq!(
//             window.moving_avg_buffer,
//             [8.5, 8.5, 7., 7.5, 19., 23.5, 11.5, 4.5, 25.5, 63., 113., 107.5, 34., 2., 2., 6.]
//         )
//     }
//     #[test]
//     fn moving_average_four() {
//         let mut window = BufferStats::new();
//         window.find_moving_avg(
//             &[
//                 12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
//             ],
//             4,
//         );
//         assert_eq!(
//             window.moving_avg_buffer,
//             [
//                 8., 8., 8., 8., 13., 15.5, 15.25, 14., 18.5, 33.75, 69.25, 85.25, 73.5, 54.75, 18.,
//                 4.
//             ]
//         )
//     }
//     #[test]
//     #[should_panic]
//     fn invalid_window_size() {
//         let mut window = BufferStats::new();
//         window.find_moving_avg(
//             &[
//                 12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
//             ],
//             1,
//         );
//     }
//     #[test]
//     #[should_panic]
//     fn window_size_too_large() {
//         let mut window = BufferStats::new();
//         window.find_moving_avg(
//             &[
//                 12., 5., 9., 6., 32., 15., 8., 1., 50., 76., 150., 65., 3., 1., 3., 9.,
//             ],
//             17,
//         );
//     }
// }
