use colored::Colorize;
use glium::glutin::event::VirtualKeyCode;
use serde::{Deserialize, Serialize};
use std::{env, fs::{File, OpenOptions, read_dir, metadata, create_dir_all, remove_file}, sync::{Arc, Mutex}, path::PathBuf, time::Duration};
use directories::UserDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEBUG: bool = env::var("DEBUG").unwrap_or("".to_string()) == "true" || false;
    pub static ref TOGGLE_KEY: VirtualKeyCode = VirtualKeyCode::Insert;

    pub static ref PROCESS_EXECUTABLE: String = "cs2.exe".to_string();
    pub static ref PROCESS_TITLE: String = "Counter-Strike 2".to_string();
    pub static ref PROCESS_CLASS: String = "SDL_app".to_string();

    pub static ref PACKAGE_NAME: String = env!("CARGO_PKG_NAME").to_string();
    pub static ref PACKAGE_VERSION: String = env!("CARGO_PKG_VERSION").to_string();
    pub static ref PACKAGE_AUTHORS: String = env!("CARGO_PKG_AUTHORS").replace(":", " & ").to_string();

    pub static ref THREAD_DELAYS: ThreadDelays = ThreadDelays {
        update_configs: Duration::from_millis(20),
        window_tasks: Duration::from_millis(10)
    };

    pub static ref CONFIG_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
    pub static ref CONFIGS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

pub struct ThreadDelays {
    pub update_configs: Duration,
    pub window_tasks: Duration
}

pub fn get_directory_dir(name: &str) -> Option<String> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            let config_dir = document_dir.join(name);

            if let Some(config_path) = config_dir.to_str() {
                return Some(config_path.to_owned());
            };
        };
    };

    return None;
}

pub fn update_configs() -> Option<String> {
    let directory_pathbuf = PathBuf::from(&*CONFIG_DIR.lock().unwrap());
    let paths = match read_dir(directory_pathbuf) {
        Ok(paths) => paths,
        _ => { return Some("DirectoryPath".to_string()); }
    };

    let mut conf_files = Vec::new();

    for path in paths {
        if let Ok(entry) = path {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".conf.json") {
                    conf_files.push(file_name.to_string());
                }
            }
        }
    }

    *CONFIGS.lock().unwrap() = conf_files;
    return None;
}

pub fn setup_config() -> Option<String> {
    let name = &*PACKAGE_NAME;
    let directory_path = match get_directory_dir(name) {
        Some(path) => path,
        _ => { return Some("FindDirectory".to_string()); }
    };

    let directory_pathbuf = PathBuf::from(&directory_path);

    if !metadata(&directory_pathbuf).is_ok() {
        match create_dir_all(&directory_pathbuf) {
            Ok(_) => {},
            _ => { return Some("CreateDirectory".to_string()); }
        };
    };

    *CONFIG_DIR.lock().unwrap() = directory_path.to_string();
    if *DEBUG { println!("{} Config Dir: {}", "[ INFO ]".blue().bold(), directory_path.bold()); }

    match update_configs() {
        Some(str) => { return Some(str); },
        _ => {}
    };

    if *DEBUG { println!("{} Configs: {}", "[ INFO ]".blue().bold(), format!("{:?}", *CONFIGS.lock().unwrap()).bold()); }

    if let Some(default_config_path) = directory_pathbuf.join("default.conf.json").to_str() {
        if (*CONFIGS.lock().unwrap()).contains(&String::from("default.conf.json")) {
            match load_config(default_config_path) {
                Ok(new_config) => { *CONFIG.lock().unwrap() = new_config; }
                Err(_) => {
                    match (*CONFIG.lock().unwrap()).save_config(default_config_path) {
                        Err(_) => { return Some("SaveDefaultConfig".to_string()); },
                        _ => {}
                    };
        
                    if *DEBUG { println!("{} {} config saved", "[ INFO ]".blue().bold(), default_config_path.bold()); }
                }
            };

            if *DEBUG { println!("{} {} config loaded", "[ INFO ]".blue().bold(), default_config_path.bold()); }
        } else {
            match (*CONFIG.lock().unwrap()).save_config(default_config_path) {
                Err(_) => { return Some("SaveDefaultConfig".to_string()); },
                _ => {}
            };

            if *DEBUG { println!("{} {} config saved", "[ INFO ]".blue().bold(), default_config_path.bold()); }
        };
    };

    return None;
}

pub fn load_config(file_path: &str) -> Result<Config, &str> {
    let file = match File::open(file_path) {
        Ok(path) => path,
        _ => { return Err("FilePath"); }
    };
    
    let config: Config = match serde_json::from_reader(file) {
        Ok(config) => config,
        _ => { return Err("ParseFile"); }
    };

    return Ok(config);
}

pub fn delete_config(file_path: &str) -> Result<bool, &str> {
    if let Err(_) = remove_file(file_path) {
        return Err("DeleteFile");
    };

    return Ok(true);
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub esp_enabled: bool,
    pub show_skeleton_esp: bool,
    pub show_head_esp: bool,
    pub head_type: usize,
    pub show_box_esp: bool,
    pub show_filled_box_esp: bool,
    pub show_health_bar: bool,
    pub show_weapon_esp: bool,
    pub show_distance: bool,
    pub smooth: f32,
    pub show_eye_ray: bool,
    pub show_player_name: bool,
    pub aim_bot: bool,
    pub aim_fov: f32,
    pub aim_position: usize,
    pub box_type: usize,
    pub box_rounding: u32,
    pub box_visible: bool,
    pub box_visible_color: (u32, u32, u32, u32),
    pub filled_box_alpha: f32,
    pub health_bar_type: usize,
    pub health_bar_rounding: u32,
    pub health_bar_first_color: (u32, u32, u32, u32),
    pub health_bar_second_color: (u32, u32, u32, u32),
    pub health_bar_third_color: (u32, u32, u32, u32),
    pub skeleton_color: (u32, u32, u32, u32),
    pub head_color: (u32, u32, u32, u32),
    pub box_color: (u32, u32, u32, u32),
    pub radar_color: (u32, u32, u32, u32),
    pub filled_box_color: (u32, u32, u32, u32),
    pub eye_ray_color: (u32, u32, u32, u32),
    pub show_radar: bool,
    pub radar_range: f32,
    pub show_radar_cross_line: bool,
    pub radar_cross_line_color: (u32, u32, u32, u32),
    pub radar_type: usize,
    pub radar_point_size_proportion: f32,
    pub radar_alpha: f32,
    pub proportion: f32,
    pub trigger_bot: bool,
    pub team_check: bool,
    pub visible_check: bool,
    pub ground_check: bool,
    pub show_head_shot_line: bool,
    pub head_shot_line_color: (u32, u32, u32, u32),
    pub aim_bot_hot_key: usize,
    pub show_snap_line: bool,
    pub trigger_delay: u32,
    pub trigger_interval: u32,
    pub rcs_bullet: u32,
    pub triggerbot_hot_key: usize,
    pub triggerbot_always: bool,
    pub aimbot_mode: usize,
    pub rcs_scale: (f32, f32),
    pub snap_line_color: (u32, u32, u32, u32),
    pub show_aim_fov_range: bool,
    pub aim_fov_range_color: (u32, u32, u32, u32),
    pub obs_bypass: bool,
    pub bunny_hop: bool,
    pub show_when_spec: bool,
    pub anti_flashbang: bool,
    pub player_name_color: (u32, u32, u32, u32),
    pub weapon_name_color: (u32, u32, u32, u32),
    pub distance_color: (u32, u32, u32, u32)
}

impl Config {
    pub fn save_config(&self, file_path: &str) -> Result<(), &str> {
        let file = match OpenOptions::new().write(true).truncate(true).create(true).open(file_path) {
            Ok(file) => file,
            _ => { return Err("CreateFile"); }
        };

        match serde_json::to_writer_pretty(file, &self) {
            Ok(_) => {},
            _ => { return Err("WriteFile"); }
        };

        return Ok(());
    }
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            esp_enabled: true,
            show_skeleton_esp: true,
            head_type: 0,
            show_head_esp: true,
            show_box_esp: true,
            show_filled_box_esp: true,
            show_health_bar: true,
            show_weapon_esp: true,
            show_distance: true,
            smooth: 0.6,
            show_eye_ray: false,
            show_player_name: true,
            aim_bot: true,
            aim_fov: 3.5,
            aim_position: 1,
            box_type: 1,
            box_rounding: 5,
            box_visible: true,
            box_visible_color: (255, 0, 0, 255),
            filled_box_alpha: 0.2,
            health_bar_type: 0,
            health_bar_rounding: 25,
            health_bar_first_color: (0, 255, 0, 255),
            health_bar_second_color: (255, 255, 0, 255),
            health_bar_third_color: (255, 0, 0, 255),
            skeleton_color: (255, 255, 255, 255),
            head_color: (255, 0, 0, 255),
            box_color: (0, 255, 255, 255),
            radar_color: (255, 0, 0, 255),
            filled_box_color: (0, 255, 255, 255),
            eye_ray_color: (255, 0, 0, 255),
            show_radar: true,
            radar_range: 148.0,
            show_radar_cross_line: false,
            radar_cross_line_color: (255, 255, 255, 255),
            radar_type: 2,
            radar_point_size_proportion: 1.0,
            radar_alpha: 0.0,
            proportion: 3100.0,
            trigger_bot: false,
            team_check: true,
            visible_check: true,
            ground_check: true,
            show_head_shot_line: false,
            head_shot_line_color: (255, 255, 255, 255),
            aim_bot_hot_key: 0,
            show_snap_line: false,
            trigger_delay: 250,
            trigger_interval: 25,
            rcs_bullet: 1,
            triggerbot_hot_key: 0,
            triggerbot_always: false,
            aimbot_mode: 0,
            rcs_scale: (1.0, 1.0),
            snap_line_color: (255, 255, 255, 255),
            show_aim_fov_range: true,
            aim_fov_range_color: (230, 230, 230, 255),
            obs_bypass: false,
            bunny_hop: false,
            show_when_spec: true,
            anti_flashbang: false,
            player_name_color: (0, 255, 255, 255),
            weapon_name_color: (255, 255, 255, 255),
            distance_color: (255, 255, 0, 255)
        };
    }
}