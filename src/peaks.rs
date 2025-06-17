use crate::bufferstats::BufferStats;

#[derive(Debug)]
pub struct Peak {
    pub start_idx: usize,
    pub end_idx: usize,
    pub peak_idx: usize,
    pub peak_magnitude: f64,
    pub fwhm: usize,
}

pub fn find_peaks(buffer: &Vec<f64>, threshold: f64, window_size: usize) -> Vec<Peak> {
    // Normalise bin magnitude
    let buffer_norm: Vec<f64> = buffer
        .iter()
        .map(|bin| bin / (buffer.len() as f64))
        .collect();

    // 1. Isolate bins in neighbourhood of local maxima
    // TO DO: Moving average calculation. Currently, mean is calculated over the whole buffer
    let mut window = BufferStats::new();
    window.mean(&buffer_norm, window_size);
    window.std_deviation(&buffer_norm);
    let peaks: Vec<(usize, &f64)> = buffer_norm
        .iter()
        .enumerate()
        .filter(|(_idx, &magnitude)| {
            magnitude - window.median.expect("median not defined")
                > window.stdev.expect("stdev not calculated") * threshold
        })
        .map(|(idx, magnitude)| (idx, magnitude))
        .collect();

    let mut peak_collection: Vec<Peak> = Vec::new();
    let mut peak_iter = peaks.chunk_by(|curr, next| next.0 - curr.0 == 1);

    let mut maxima_neighbourhood = peak_iter.next();

    while maxima_neighbourhood != None {
        let local_max_start = maxima_neighbourhood.unwrap().first().unwrap().0;
        let local_max_end = maxima_neighbourhood.unwrap().last().unwrap().0;
        let mut max_val = 0.;
        let mut max_idx = 0;

        for sample in maxima_neighbourhood.unwrap() {
            if *sample.1 > max_val {
                max_val = *sample.1;
                max_idx = sample.0;
            }
        }
        peak_collection.push(Peak {
            start_idx: (local_max_start),
            end_idx: (local_max_end),
            peak_idx: (max_idx),
            peak_magnitude: (max_val),
            fwhm: (local_max_end - local_max_start),
        });
        maxima_neighbourhood = peak_iter.next();
    }
    peak_collection
}
