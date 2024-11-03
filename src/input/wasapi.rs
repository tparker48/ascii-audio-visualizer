use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamError};
use std::sync::{Arc, Mutex};

use crate::audio_process_buffer::AudioProcessBuffer;


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

pub fn connect() -> Result<Arc<Mutex<AudioProcessBuffer>>, anyhow::Error>  {
    let process_buffer_writer = Arc::new(Mutex::new(AudioProcessBuffer::new()));
    let process_buffer_reader = process_buffer_writer.clone();

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Failed to get default output device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| cpal_audio_callback::<[i8]>(audio_buffer, &process_buffer_writer),
            move |err| cpal_err_callback(err),
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| cpal_audio_callback::<[i16]>(audio_buffer, &process_buffer_writer),
            move |err| cpal_err_callback(err),
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| cpal_audio_callback::<[i32]>(audio_buffer, &process_buffer_writer),
            move |err| cpal_err_callback(err),
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| cpal_audio_callback::<[f32]>(audio_buffer, &process_buffer_writer),
            move |err| cpal_err_callback(err),
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };
    
    stream.play()?;
    return Ok(process_buffer_reader);
}


pub fn cpal_audio_callback<T: AsF32Audio + ?Sized>(input_buffer: &T, processing_buffer: &Arc<Mutex<AudioProcessBuffer>>)
{
    // as_f32_samples audio format to f32
   let input_buffer = (*input_buffer).as_f32_samples(); 
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

pub fn cpal_err_callback(err: StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

