# cavii
Inspired by CAVA, CAVII is a configurable ASCII-based audio visualizer that runs in-terminal

![sine](https://github.com/user-attachments/assets/898bb93e-7b39-461b-8e39-4f1cb6501213)  |  ![wavy](https://github.com/user-attachments/assets/3ac3d87b-2314-4bbb-ac3b-0c28c7dc414a)
:-------------------------:|:-------------------------:
![eq_mountains](https://github.com/user-attachments/assets/68fbb590-140a-44dd-8ef6-c65d2a6c68b3)  |  ![spectrum](https://github.com/user-attachments/assets/2aade753-6ec8-4aed-a93a-c598a90b8cb8)

## Installation
Dependencies:
 - Cargo
 - (Linux only) ALSA and PulseAudio libraries: 
`sudo apt-get install libasound2-dev libpulse-dev`

Clone repo, then build:
```
cd ascii-audio-visualizer
cargo build
```

## Configuration
See config.ini for default config example

## Usage
Run with default config file:
```
cargo run
```
Run with custom config file:
```
cargo run -- -c [CONFIG_FILE_PATH]
```

## Contributing
Pull requests are welcome! 

### Areas for improvement
 - support for more linux audio hosts (pipewire, jack, etc)
 - performance improvements to print/draw logic
 - new animations
 - new audio features
 - other?  
