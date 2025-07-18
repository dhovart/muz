use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};
use spectrum_analyzer::windows::hann_window;

pub struct SpectrumAnalyzer {
    buffer: Vec<f32>,
    fft_size: usize,
    sample_rate: f32,
}

impl SpectrumAnalyzer {
    pub fn new(fft_size: usize, sample_rate: f32) -> Self {
        Self {
            buffer: Vec::with_capacity(fft_size),
            fft_size,
            sample_rate,
        }
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.buffer.push(sample);
            if self.buffer.len() >= self.fft_size {
                self.buffer.drain(0..self.buffer.len() - self.fft_size);
            }
        }
    }

    pub fn get_spectrum(&self) -> Vec<f32> {
        if self.buffer.len() < self.fft_size {
            return vec![0.0; 64]; // Return empty spectrum with 64 bins
        }

        let windowed_samples = hann_window(&self.buffer[self.buffer.len() - self.fft_size..]);
        
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
                *bin = sum / (end_idx - start_idx) as f32;
            }
        }

        bins
    }
}