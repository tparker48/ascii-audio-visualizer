
use std::sync::{Arc, Mutex};
use std::thread;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};
use pulse::def::BufferAttr;

use crate::audio_process_buffer::AudioProcessBuffer;

const BUFFER_SIZE: usize = 1024;

pub fn connect() -> Result<Arc<Mutex<AudioProcessBuffer>>, anyhow::Error> {
    let process_buffer_writer = Arc::new(Mutex::new(AudioProcessBuffer::new()));
    let process_buffer_reader = process_buffer_writer.clone();
    thread::spawn(move || { audio_listener(process_buffer_writer);});

    return Ok(process_buffer_reader);
}


fn audio_listener(shared_buffer: Arc<Mutex<AudioProcessBuffer>>) {
    let spec = Spec {
        format: Format::FLOAT32NE,
        channels: 1,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let attributes = BufferAttr{
        maxlength: BUFFER_SIZE as u32,
        tlength: 0,
        prebuf: 0,
        minreq: 0,
        fragsize: BUFFER_SIZE as u32
    };

    let s = Simple::new(
        None,                // Use the default server
        "Audio Listener",            // Our application’s name
        Direction::Record, // We want a playback stream
        Some("53"),             // TODO get default sink and append ".listener"
        "listener",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        Some(&attributes)
        ).unwrap();


    let mut raw_buffer = [0 as u8; BUFFER_SIZE*4];
    loop{
        // capture raw bytes
        s.read(&mut raw_buffer).unwrap();

        // convert to f32 format
        match shared_buffer.try_lock() {
            Ok(mut buffer) => {
                for sample in audio_samples_from_raw_bytes(&raw_buffer){
                    buffer.push(sample);
                }
            },
            Err(_) => {continue;}
        }
    }
}

fn audio_samples_from_raw_bytes(raw_bytes: &[u8;BUFFER_SIZE*4]) -> Vec<f32> {
    raw_bytes
        .chunks(4)
        .map(|bytes: &[u8]| {
            f32::from_ne_bytes(
                [bytes[0], bytes[1], bytes[2], bytes[3]]
            )
        })
        .collect()
}