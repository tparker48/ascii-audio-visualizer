use configparser::ini::Ini;

use crate::colors::{Color, FromHex}; 

pub struct Config {
    // Colors
    pub color_1: Color,
    pub color_2: Color,
    pub color_3: Color,
    pub bg_color: Color,
    pub bg_alt_color: Color,

    // Animation Settings
    pub animation_length: i64,

    // Animations
    pub animations: Vec<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Config {
        let mut config = Ini::new();
        config.load(config_path).expect("Could not open config path!");

        let color_1 = config.get("colors", "color_1")
                            .expect("Error: 'color_1' key not found in config."); 
        let color_2 = config.get("colors", "color_2")
                            .expect("Error: 'color_2' key not found in config."); 
        let color_3 = config.get("colors", "color_3")
                            .expect("Error: 'color_3' key not found in config.");
        let bg_color = config.get("colors", "background")
                            .expect("Error: 'background' key not found in config.");
        let bg_color_alt = config.get("colors", "background-alt")
                            .expect("Error: 'background-alt' key not found in config.");

        let animation_length = config.getint("animation-settings", "duration_s")
                                     .expect("Error: 'duration_s' key not found in config.")
                                     .expect("Error: 'duration_s value invalid.");

        let map = config.get_map().expect("Error parsing config.ini."); 
        let animators = map.get("animations").expect("Coult not find 'animations' in config.");
        let mut animators: Vec<(String, i32)> = animators
                                                    .iter()
                                                    .map(|(key,val)| {
                                                        (key.clone(), val.clone().unwrap().parse::<i32>().unwrap())
                                                    })
                                                    .filter(|(key,val)| *val > 0)
                                                    .collect();
        animators.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        let animations: Vec<String> = animators
                                        .iter()
                                        .map(|(key, _val)| key.clone())
                                        .collect();
        Config {
            color_1: Color::from_hex_string(color_1).expect("Invalid Hex!"),
            color_2: Color::from_hex_string(color_2).expect("Invalid Hex!"),
            color_3: Color::from_hex_string(color_3).expect("Invalid Hex!"),
            bg_color: Color::from_hex_string(bg_color).expect("Invalid Hex!"),
            bg_alt_color: Color::from_hex_string(bg_color_alt).expect("Invalid Hex!"),
            animation_length: animation_length,
            animations: animations 
        }
    }
}