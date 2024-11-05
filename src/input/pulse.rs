
use std::sync::{Arc, Mutex};
use std::thread;
use std::ops::Deref;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};
use pulse::def::{BufferAttr, Retval};
use pulse::mainloop::standard::Mainloop;
use pulse::context::{Context, FlagSet as ContextFlagSet};
use std::rc::Rc;
use std::cell::RefCell;


use crate::audio_process_buffer::AudioProcessBuffer;

const BUFFER_SIZE: usize = 1024;

pub fn connect() -> Result<Arc<Mutex<AudioProcessBuffer>>, anyhow::Error> {
    let name = get_default_sink_name();
    println!("{}", name);

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


    let mut default_sink_monitor = get_default_sink_name();
    default_sink_monitor.push_str(".monitor");

    let s = Simple::new(
        None,                // Use the default server
        "Audio Listener",            // Our application’s name
        Direction::Record, // We want a playback stream
        Some(default_sink_monitor.as_str()),             // TODO get default sink and append ".monitor"
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
    /* Converts an array of raw bytes into a vector of f32 samples
     */
    raw_bytes
        .chunks(4)
        .map(|bytes: &[u8]| {
            f32::from_ne_bytes(
                [bytes[0], bytes[1], bytes[2], bytes[3]]
            )
        })
        .collect()
}

fn get_default_sink_name() -> String {
    /* Finds the name of default sink (audio output device)
        1. Start pulse audio context,
        2. Query the context's server for default sink name
        3. Return default sink name
    */
    let mut mainloop = Rc::new(RefCell::new(Mainloop::new()
        .expect("Failed to create mainloop")));

    let mut context = Rc::new(RefCell::new(Context::new(
        mainloop.borrow().deref(),
        "Default Audio Sink Context",
        ).expect("Failed to create new context")));

    context.borrow_mut().connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        mainloop.borrow_mut().iterate(true);
        match context.borrow().get_state() {
            pulse::context::State::Ready => { break; },
            pulse::context::State::Failed |
            pulse::context::State::Terminated => {
                eprintln!("Context state failed/terminated, quitting...");
                return String::from("Failed");
            },
            _ => {},
        }
    }

    let name_write_ptr = Arc::new(Mutex::new(String::new()));
    let name_read_ptr = name_write_ptr.clone();

    let operation = context.borrow().introspect().get_server_info(move |info|{
        let mut name_ref = name_write_ptr.lock().unwrap();
        *name_ref = String::from(
            info.default_sink_name
                .as_ref()
                .unwrap()
                .deref()
        );
            
    });
    while operation.get_state() != pulse::operation::State::Done {
        mainloop.borrow_mut().iterate(true);
    } 

    // Clean shutdown
    mainloop.borrow_mut().quit(Retval(0)); 
    
    let name_ref = name_read_ptr.lock().unwrap();
    return name_ref.clone();
}
