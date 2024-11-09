// Compile with pulse bindings on linux, to listen via pulseaudio
#[cfg(
    any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
pub mod pulse;


// Compile with CPAL on windows, to listen via WASAPI 
#[cfg(target_os = "windows")]
pub mod wasapi;

