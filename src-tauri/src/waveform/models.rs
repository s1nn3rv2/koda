use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformData {
    pub samples: Vec<f32>, // normalized amplitude 0.0 to 1.0
    pub duration: f64,
}
