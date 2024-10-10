use std::sync::{Arc, Mutex};
use rustfft::{num_complex::Complex, FftPlanner};
use cpal::StreamError;

const BUFFER_SIZE: usize = 512;
const FFT_SIZE: usize = 512;
const SMOOTHING_SIZE: usize = 9;
// const FS: usize = 48000;


pub struct AudioProcessBuffer {
    buffer: [f32; BUFFER_SIZE],
    head: usize,
    fft: Arc<dyn rustfft::Fft<f32>>,

    pub rms: SmoothedValue,
    pub zcr: SmoothedValue
}

impl AudioProcessBuffer {
    pub fn new() -> AudioProcessBuffer {
        let mut planner = FftPlanner::new();

        return AudioProcessBuffer{
            buffer: [0.0; BUFFER_SIZE],
            head: 0,
            fft: planner.plan_fft_forward(FFT_SIZE),
            rms: SmoothedValue::new(0.0, true),
            zcr: SmoothedValue::new(0.0, false),
        }
    }

    pub fn remaining_cap(self: &AudioProcessBuffer) -> usize {
        return self.buffer.len() - self.head;
    }

    pub fn push(self: &mut AudioProcessBuffer, value: f32) {
        if self.remaining_cap() == 0 {
            self.process_full_buffer();
        }
        self.buffer[self.head] = value;
        self.head+=1;   
    }

    pub fn process_full_buffer(self: &mut AudioProcessBuffer) {
        self.head = 0;

        // Zero Crossing Rate
        let mut zero_crosses = 0;
        let mut prev_sample = self.buffer[0];
        for &sample in self.buffer.iter(){
            if (sample > 0.0 && prev_sample < 0.0) || (sample < 0.0 && prev_sample > 0.0) {
                zero_crosses+=1;
            }
            prev_sample = sample;
        }
        let zcr = (zero_crosses as f32) / (self.buffer.len() as f32);
        self.zcr.write(zcr);

        // Root Mean Squared Energy
        let sum_of_squares: f32 = self.buffer.iter().map(|number| number*number).sum();
        let rms = (sum_of_squares / self.buffer.len() as f32).sqrt();
        self.rms.write(rms);
        
        // FFT
        let mut fft_buffer: Vec<Complex<f32>> = self.buffer.iter().map(|r|{
            Complex::from(r)
        }).collect();
        self.fft.process(&mut fft_buffer);
        

        //let freq_max = (max_idx as f32)*((FS/2) as f32)/((FFT_SIZE) as f32);
    }
}

pub struct SmoothedValue {
    buffer: [f32; SMOOTHING_SIZE],
    head: usize,
    adaptive_min: f32,
    adaptive_max: f32,
    min: f32,
    max: f32,
    adaptive: bool,
    pub smoothed_val: f32,
}

impl SmoothedValue {
    fn new(starter_value: f32, adaptive: bool) -> SmoothedValue {
        SmoothedValue{
            buffer: [starter_value; SMOOTHING_SIZE], 
            head: 0, 
            smoothed_val: starter_value,
            adaptive_min: starter_value, 
            adaptive_max:starter_value,
            min: starter_value, 
            max:starter_value,
            adaptive: adaptive}
    }

    fn write(self: &mut SmoothedValue, value: f32) {
        let mut value = value;

        // Check min/max
        self.max = self.max.max(value);
        self.min = self.min.min(value);
        self.adaptive_max = self.adaptive_max.max(value);
        self.adaptive_min = self.adaptive_min.min(value);

        if self.adaptive  && self.adaptive_max > 0.0 {
            value = (value-self.adaptive_min) / (self.adaptive_max-self.adaptive_min);
        } 

        // Set
        self.head = (self.head+1)%SMOOTHING_SIZE;
        self.smoothed_val -= self.buffer[self.head];
        self.buffer[self.head] = value / (SMOOTHING_SIZE as f32);
        self.smoothed_val += self.buffer[self.head];

         // Decay min/max
        self.adaptive_max *= 0.95;
        self.adaptive_min *= 1.05;

        // Don't let adaptive min/max decay too much
        self.adaptive_max = self.adaptive_max.max(self.max*0.01);
        self.adaptive_min = self.adaptive_min.min(self.min*1.99);
    }
}


// Callback for audio thread
pub fn audio_callback(input_buffer: &[f32], processing_buffer: &Arc<Mutex<AudioProcessBuffer>>)
{
    // write to process buffer in mono
    match processing_buffer.try_lock() {
        Ok(mut buffer) => {
            for i in 0..input_buffer.len() {
                if i%2 == 1 {
                    buffer.push((input_buffer[i] + input_buffer[i-1]) / 2.0);
                }
            }
        },
        Err(_) => {}
    }
}

pub fn err_callback(err: StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}