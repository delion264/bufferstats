use crate::bufferstats::BufferStats;
use ringbuffer::{AllocRingBuffer, RingBuffer};

#[derive(Debug)]
pub struct Peak {
    pub start_idx: usize,
    pub end_idx: usize,
    pub peak_idx: usize,
    pub peak_magnitude: f64,
    pub peak_width: usize,
}

pub struct Peakfinder {
    pub peak_indices: Vec<usize>,
    pub peaks: Vec<Peak>,
    pub processing_interval: usize,
    pub window_size: usize,
    pub peak_weight: f64,
    pub weighted_stats: BufferStats,
    pub weighted_data: AllocRingBuffer<f64>,
}

impl Peakfinder {
    pub fn new(&mut self, proc_int: usize, window_len: usize, weight: f64) -> Self {
        Self {
            peak_indices: Vec::new(),
            peaks: Vec::new(),
            processing_interval: proc_int, // Think buffer length in terms of FFT size
            window_size: window_len,
            peak_weight: weight,
            weighted_stats: BufferStats::new(self.processing_interval, self.window_size),
            weighted_data: AllocRingBuffer::new(self.processing_interval),
        }
    }

    pub fn index_peaks(&mut self, data: &[f64], threshold: f64) {
        assert_eq!(data.len(), self.processing_interval);
        for idx in 0..data.len() {
            self.weighted_data.push(data[idx]);
        }
        self.weighted_stats.init_moving_avg(&data);
        self.weighted_stats.init_moving_variance(&data);
        self.weighted_stats.init_moving_max(&data);

        for idx in 0..data.len() {
            if data[idx]
                > self.weighted_stats.moving_avg[idx - 1]
                    + threshold * self.weighted_stats.moving_max[idx - 1]
            {
                self.peak_indices.push(idx);
                self.weighted_data[idx] = (1. - self.peak_weight) * self.weighted_data[idx - 1]
                    + self.peak_weight * self.weighted_data[idx];
                // Mark signal indices
            }
            self.weighted_stats
                .update_moving_avg(&self.weighted_data.to_vec());
            self.weighted_stats
                .update_moving_variance(&self.weighted_data.to_vec());
            self.weighted_stats
                .update_moving_max(&self.weighted_data.to_vec());
        }
        let mut peak_iter = self.peak_indices.chunk_by(|curr, next| next - curr == 1);
        let mut maxima_neighbourhood = peak_iter.next();

        while maxima_neighbourhood != None {
            let local_max_start = maxima_neighbourhood.unwrap().first().unwrap();
            let local_max_end = maxima_neighbourhood.unwrap().last().unwrap();
            let mut max_val = 0.;
            let mut max_idx = 0;

            for idx in maxima_neighbourhood.unwrap() {
                if data[*idx] > max_val {
                    max_val = data[*idx];
                    max_idx = *idx;
                }
                self.peaks.push(Peak {
                    start_idx: *local_max_start,
                    end_idx: *local_max_end,
                    peak_idx: (max_idx),
                    peak_magnitude: (max_val),
                    peak_width: (local_max_end - local_max_start),
                });
                maxima_neighbourhood = peak_iter.next();
            }
        }
    }
}
