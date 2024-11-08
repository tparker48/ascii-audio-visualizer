// Implements logic for converting buffers of different audio sample types to Vec<f32>

pub trait AsF32Audio {
    fn as_f32_samples(&self) -> Vec<f32>;
}

impl AsF32Audio for [i8] {
    fn as_f32_samples(&self) -> Vec<f32> {
        self
         .iter()
         .map(|i|(*i as f32) / (i8::MAX as f32))
         .collect()
    }
}

impl AsF32Audio for [i16] {
    fn as_f32_samples(&self) -> Vec<f32> {
        self
          .iter()
          .map(|i|(*i as f32) / (i16::MAX as f32))
          .collect()
    }
}

impl AsF32Audio for [i32] {
    fn as_f32_samples(&self) -> Vec<f32> {
        self
          .iter()
          .map(|i|(*i as f32) / (i32::MAX as f32))
          .collect()
    }
}

impl AsF32Audio for [f32] {
    fn as_f32_samples(&self) -> Vec<f32> {
        self
          .iter()
          .map(|f|*f)
          .collect()
    }
}

impl AsF32Audio for [u8] {
    // Interprets each u8 as a byte, with each group of 4 comprising one f32
    fn as_f32_samples(&self) -> Vec<f32> {
        self
            .chunks(4)
            .map(|bytes: &[u8]| {
                f32::from_ne_bytes(
                    [bytes[0], bytes[1], bytes[2], bytes[3]]
                )
            })
            .collect()
    }
}
