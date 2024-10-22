// Implements logic for converting all possible audio sample buffer types (i8,i16,i32) to Vec<f32>

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
