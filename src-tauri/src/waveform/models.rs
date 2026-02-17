use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformData {
    pub samples: Vec<f32>, // normalized amplitude 0.0 to 1.0
    pub duration: f64,
}

impl WaveformData {
    pub fn new(samples: Vec<f32>, duration: f64) -> Self {
        Self { samples, duration }
    }

    pub fn empty(num_samples: usize) -> Self {
        Self {
            samples: vec![0.0; num_samples],
            duration: 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn max_amplitude(&self) -> f32 {
        self.samples
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
    }

    pub fn normalize(&mut self) {
        let max = self.max_amplitude();
        if max > 0.0 {
            for sample in &mut self.samples {
                *sample /= max;
            }
        }
    }
}
