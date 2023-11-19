use glutin::event::VirtualKeyCode;
use mki::Keyboard;
use serde::{Deserialize, Serialize};
use std::{env, fs::{File, OpenOptions, read_dir, metadata, create_dir_all, remove_file}, sync::{Arc, Mutex}, path::PathBuf, time::Duration};
use directories::UserDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PACKAGE_NAME: String = "ProExt".to_string();
    pub static ref PACKAGE_VERSION: String = env!("CARGO_PKG_VERSION").to_string();
    pub static ref PACKAGE_AUTHORS: String = env!("CARGO_PKG_AUTHORS").replace(":", " & ").to_string();

    pub static ref UPDATE_URL: String = "https://github.com/vytrol/ProExt/raw/main/bin/proext.exe".to_string();
    pub static ref UPDATE_HASH_URL: String = "https://github.com/vytrol/ProExt/raw/main/bin/hash.txt".to_string();

    pub static ref RPC_CLIENT_ID: u64 = 1174845327099048018;
    pub static ref RPC_STATE: String = "An open-source, external CS2 cheat.".to_string();
    pub static ref RPC_IMAGE_ASSET: String = "cs2".to_string();

    pub static ref TOGGLE_KEY: VirtualKeyCode = VirtualKeyCode::Insert;
    pub static ref TOGGLE_KEY_MKI: Keyboard = Keyboard::Insert;

    pub static ref PROCESS_EXECUTABLE: String = "cs2.exe".to_string();
    pub static ref PROCESS_TITLE: String = "Counter-Strike 2".to_string();
    pub static ref PROCESS_CLASS: String = "SDL_app".to_string();

    pub static ref THREAD_DELAYS: ThreadDelays = ThreadDelays {
        update_configs: Duration::from_millis(10),
        window_tasks: Duration::from_millis(10),
        io_tasks: Duration::from_millis(1),
        rpc: Duration::from_millis(10)
    };

    pub static ref CHEAT_DELAYS: CheatDelays = CheatDelays {
        aimbot: Duration::from_millis(10)
    };

    pub static ref CONFIG_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
    pub static ref CONFIGS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

pub struct ThreadDelays {
    pub update_configs: Duration,
    pub window_tasks: Duration,
    pub io_tasks: Duration,
    pub rpc: Duration
}

pub struct CheatDelays {
    pub aimbot: Duration
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

    match update_configs() {
        Some(str) => { return Some(str); },
        _ => {}
    };

    if let Some(default_config_path) = directory_pathbuf.join("default.conf.json").to_str() {
        if (*CONFIGS.lock().unwrap()).contains(&String::from("default.conf.json")) {
            match load_config(default_config_path) {
                Ok(new_config) => { *CONFIG.lock().unwrap() = new_config; }
                Err(_) => {
                    match (*CONFIG.lock().unwrap()).save_config(default_config_path) {
                        Err(_) => { return Some("SaveDefaultConfig".to_string()); },
                        _ => {}
                    };
                }
            };
        } else {
            match (*CONFIG.lock().unwrap()).save_config(default_config_path) {
                Err(_) => { return Some("SaveDefaultConfig".to_string()); },
                _ => {}
            };
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
pub struct ESP {
    pub enabled: bool,
    pub box_enabled: bool,
    pub box_color: (u32, u32, u32, u32),
    pub box_mode: usize,
    pub box_rounding: u32,
    pub box_target_enabled: bool,
    pub box_target_color: (u32, u32, u32, u32),
    pub filled_box_enabled: bool,
    pub filled_box_color: (u32, u32, u32, u32),
    pub filled_box_alpha: f32,
    pub skeleton_enabled: bool,
    pub skeleton_color: (u32, u32, u32, u32),
    pub head_enabled: bool,
    pub head_color: (u32, u32, u32, u32),
    pub head_mode: usize,
    pub eye_ray_enabled: bool,
    pub eye_ray_color: (u32, u32, u32, u32),
    pub health_bar_enabled: bool,
    pub health_bar_first_color: (u32, u32, u32, u32),
    pub health_bar_second_color: (u32, u32, u32, u32),
    pub health_bar_third_color: (u32, u32, u32, u32),
    pub health_bar_mode: usize,
    pub health_bar_rounding: u32,
    pub name_enabled: bool,
    pub name_color: (u32, u32, u32, u32),
    pub weapon_name_enabled: bool,
    pub weapon_name_color: (u32, u32, u32, u32),
    pub distance_enabled: bool,
    pub distance_color: (u32, u32, u32, u32),
    pub bomb_enabled: bool,
    pub bomb_color: (u32, u32, u32, u32),
    pub bomb_rounding: u32,
    pub filled_bomb_enabled: bool,
    pub filled_bomb_color: (u32, u32, u32, u32),
    pub filled_bomb_alpha: f32,
    pub snap_line_enabled: bool,
    pub snap_line_color: (u32, u32, u32, u32),
    pub snap_line_mode: usize
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Aimbot {
    pub enabled: bool,
    pub key: usize,
    pub mode: usize,
    pub fov_circle_enabled: bool,
    pub fov_circle_color: (u32, u32, u32, u32),
    pub fov_circle_target_enabled: bool,
    pub fov_circle_target_color: (u32, u32, u32, u32),
    pub fov_circle_outline_enabled: bool,
    pub only_visible: bool,
    pub only_grounded: bool,
    pub bone: usize,
    pub fov: f32,
    pub smooth: f32
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Triggerbot {
    pub enabled: bool,
    pub key: usize,
    pub mode: usize,
    pub tap_interval: u32,
    pub always_activated: bool,
    pub delay: u32
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Crosshair {
    pub enabled: bool,
    pub color: (u32, u32, u32, u32),
    pub target_enabled: bool,
    pub target_color: (u32, u32, u32, u32),
    pub outline_enabled: bool,
    pub dot_enabled: bool,
    pub dot_size: u32,
    pub circle_enabled: bool,
    pub circle_radius: u32,
    pub lines_enabled: bool,
    pub lines_width: u32,
    pub lines_height: u32,
    pub lines_space: u32,
    pub lines_thickness: u32
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Radar {
    pub enabled: bool,
    pub color: (u32, u32, u32, u32),
    pub mode: usize,
    pub alpha: f32,
    pub crossline_enabled: bool,
    pub crossline_color: (u32, u32, u32, u32),
    pub point_size: f32,
    pub proportion: f32,
    pub range: f32
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Misc {
    pub enabled: bool,
    pub watermark_enabled: bool,
    pub cheat_list_enabled: bool,
    pub bomb_timer_enabled: bool,
    pub bomb_timer_color_disabled: (u32, u32, u32, u32),
    pub bomb_timer_color_enabled: (u32, u32, u32, u32),
    pub spectator_list_enabled: bool,
    pub spectator_list_color: (u32, u32, u32, u32),
    pub exclude_team: bool,
    pub show_on_spectate: bool,
    pub bypass_capture: bool,
    pub headshot_line_enabled: bool,
    pub headshot_line_color: (u32, u32, u32, u32),
    pub discord_rpc_enabled: bool,
    pub no_flash_enabled: bool,
    pub bunny_hop_enabled: bool
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindowPositions {
    pub menu: WindowPosition,
    pub watermark: WindowPosition,
    pub cheat_list: WindowPosition,
    pub bomb_timer: WindowPosition,
    pub spectator_list: WindowPosition
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub esp: ESP,
    pub aimbot: Aimbot,
    pub triggerbot: Triggerbot,
    pub crosshair: Crosshair,
    pub radar: Radar,
    pub misc: Misc,
    pub window_positions: WindowPositions
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
            esp: ESP {
                enabled: true,
                box_enabled: true,
                box_color: (0, 255, 255, 255),
                box_mode: 1,
                box_rounding: 5,
                box_target_enabled: true,
                box_target_color: (255, 0, 0, 255),
                filled_box_enabled: true,
                filled_box_color: (0, 255, 255, 255),
                filled_box_alpha: 0.2,
                skeleton_enabled: true,
                skeleton_color: (255, 255, 255, 255),
                head_enabled: true,
                head_color: (255, 0, 0, 255),
                head_mode: 0,
                eye_ray_enabled: false,
                eye_ray_color: (255, 0, 0, 255),
                health_bar_enabled: true,
                health_bar_first_color: (0, 255, 0, 255),
                health_bar_second_color: (255, 255, 0, 255),
                health_bar_third_color: (255, 0, 0, 255),
                health_bar_mode: 0,
                health_bar_rounding: 25,
                name_enabled: true,
                name_color: (0, 255, 255, 255),
                weapon_name_enabled: true,
                weapon_name_color: (255, 255, 255, 255),
                distance_enabled: true,
                distance_color: (255, 255, 0, 255),
                bomb_enabled: true,
                bomb_color: (0, 255, 255, 255),
                bomb_rounding: 5,
                filled_bomb_enabled: true,
                filled_bomb_color: (0, 255, 255, 255),
                filled_bomb_alpha: 0.2,
                snap_line_enabled: false,
                snap_line_color: (255, 255, 255, 255),
                snap_line_mode: 1
            },
            aimbot: Aimbot {
                enabled: true,
                key: 0,
                mode: 0,
                fov_circle_enabled: true,
                fov_circle_color: (0, 255, 0, 255),
                fov_circle_target_enabled: true,
                fov_circle_target_color: (255, 0, 0, 255),
                fov_circle_outline_enabled: true,
                only_visible: true,
                only_grounded: true,
                bone: 1,
                fov: 3.5,
                smooth: 1.5
            },
            triggerbot: Triggerbot {
                enabled:  false,
                key: 0,
                mode: 0,
                tap_interval: 150,
                always_activated: false,
                delay: 250
            },
            crosshair: Crosshair {
                enabled: true,
                color: (0, 255, 0, 255),
                target_enabled: true,
                target_color: (255, 0, 0, 255),
                outline_enabled: true,
                dot_enabled: true,
                dot_size: 1,
                circle_enabled: true,
                circle_radius: 5,
                lines_enabled: true,
                lines_width: 9,
                lines_height: 9,
                lines_space: 7,
                lines_thickness: 1
            },
            radar: Radar {
                enabled: false,
                color: (255, 0, 0, 255),
                mode: 2,
                alpha: 0.0,
                crossline_enabled: false,
                crossline_color: (255, 255, 255, 255),
                point_size: 1.0,
                proportion: 3100.0,
                range: 148.0
            },
            misc: Misc {
                enabled: true,
                watermark_enabled: true,
                cheat_list_enabled: true,
                bomb_timer_enabled: false,
                bomb_timer_color_disabled: (0, 255, 255, 255),
                bomb_timer_color_enabled: (255, 0, 0, 255),
                spectator_list_enabled: false,
                spectator_list_color: (0, 255, 255, 255),
                exclude_team: true,
                show_on_spectate: true,
                bypass_capture: true,
                headshot_line_enabled: false,
                headshot_line_color: (255, 255, 255, 255),
                discord_rpc_enabled: true,
                no_flash_enabled: false,
                bunny_hop_enabled: false
            },
            window_positions: WindowPositions {
                menu: WindowPosition { x: 600.0, y: 150.0 },
                watermark: WindowPosition { x: 300.0, y: 5.0 },
                cheat_list: WindowPosition { x: 300.0, y: 58.0 },
                bomb_timer: WindowPosition { x: 30.0, y: 330.0 },
                spectator_list: WindowPosition { x: 460.0, y: 58.0 }
            }
        };
    }
}