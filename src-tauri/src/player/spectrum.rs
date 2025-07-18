use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};
use spectrum_analyzer::windows::hann_window;

pub struct SpectrumAnalyzer {
    buffer: Vec<f32>,
    fft_size: usize,
    sample_rate: f32,
    write_index: usize,
    filled: bool,
}

impl SpectrumAnalyzer {
    pub fn new(fft_size: usize, sample_rate: f32) -> Self {
        Self {
            buffer: vec![0.0; fft_size],
            fft_size,
            sample_rate,
            write_index: 0,
            filled: false,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_index = 0;
        self.filled = false;
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.buffer[self.write_index] = sample;
            self.write_index = (self.write_index + 1) % self.fft_size;
            
            if self.write_index == 0 {
                self.filled = true;
            }
        }
    }

    pub fn get_spectrum(&self) -> Vec<f32> {
        if !self.filled {
            return vec![0.0; 64]; // Return empty spectrum with 64 bins
        }

        // Create a properly ordered buffer from the circular buffer
        // We want the oldest samples first, so start from write_index (oldest)
        let mut ordered_buffer = Vec::with_capacity(self.fft_size);
        for i in 0..self.fft_size {
            let idx = (self.write_index + i) % self.fft_size;
            ordered_buffer.push(self.buffer[idx]);
        }

        let windowed_samples = hann_window(&ordered_buffer);
        
        let spectrum = samples_fft_to_spectrum(
            &windowed_samples,
            self.sample_rate as u32,
            FrequencyLimit::Max(22050.0),
            None,
        );

        match spectrum {
            Ok(spectrum) => self.spectrum_to_bins(spectrum, 64),
            Err(_) => vec![0.0; 64],
        }
    }

    fn spectrum_to_bins(&self, spectrum: FrequencySpectrum, num_bins: usize) -> Vec<f32> {
        let mut bins = vec![0.0; num_bins];
        let spectrum_data = spectrum.data();
        
        if spectrum_data.is_empty() {
            return bins;
        }

        let bin_size = spectrum_data.len() / num_bins;
        
        for (i, bin) in bins.iter_mut().enumerate() {
            let start_idx = i * bin_size;
            let end_idx = ((i + 1) * bin_size).min(spectrum_data.len());
            
            if start_idx < spectrum_data.len() {
                let sum: f32 = spectrum_data[start_idx..end_idx]
                    .iter()
                    .map(|(_, magnitude)| magnitude.val())
                    .sum();
                let avg = sum / (end_idx - start_idx) as f32;
                // Normalize to a reasonable range (0.0 to 1.0)
                *bin = (avg * 0.01).min(1.0); // Scale down the values
            }
        }

        bins
    }
}