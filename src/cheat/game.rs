use std::sync::{Arc, Mutex};

use colored::Colorize;
use mint::Vector2;
use lazy_static::lazy_static;

use crate::utils::process_manager::{get_process_module_handle, read_memory_auto, write_memory_auto};
use crate::cheat::offsets::{ENTITY_LIST, MATRIX, VIEW_ANGLE, LOCAL_PLAYER_CONTROLLER, LOCAL_PLAYER_PAWN, FORCE_JUMP, GLOBAL_VARS};
use crate::utils::config::DEBUG;
use crate::cheat::view::View;

lazy_static! {
    pub static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
        address: Address {
            client_dll: 0,
            entity_list: 0,
            matrix: 0,
            view_angle: 0,
            entity_list_entry: 0,
            local_controller: 0,
            local_pawn: 0,
            force_jump: 0,
            global_vars: 0
        },
        view: View {
            matrix: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }));
}

pub struct Game {
    pub address: Address,
    pub view: View
}

pub struct Address {
    pub client_dll: u64,
    pub entity_list: u64,
    pub matrix: u64,
    pub view_angle: u64,
    pub entity_list_entry: u64,
    pub local_controller: u64,
    pub local_pawn: u64,
    pub force_jump: u64,
    pub global_vars: u64
}

pub fn init_game_address() -> bool {
    let mut game = GAME.lock().unwrap();
    let entity_list = ENTITY_LIST.lock().unwrap();
    let matrix = MATRIX.lock().unwrap();
    let view_angle = VIEW_ANGLE.lock().unwrap();
    let local_player_controller = LOCAL_PLAYER_CONTROLLER.lock().unwrap();
    let local_player_pawn = LOCAL_PLAYER_PAWN.lock().unwrap();
    let force_jump = FORCE_JUMP.lock().unwrap();
    let global_vars = GLOBAL_VARS.lock().unwrap();

    (*game).address.client_dll = get_process_module_handle("client.dll") as u64;

    (*game).address.entity_list = (*game).address.client_dll + *entity_list as u64;
    (*game).address.matrix = (*game).address.client_dll + *matrix as u64;
    (*game).address.view_angle = (*game).address.client_dll + *view_angle as u64;
    (*game).address.local_controller = (*game).address.client_dll + *local_player_controller as u64;
    (*game).address.local_pawn = (*game).address.client_dll + *local_player_pawn as u64;
    (*game).address.force_jump = (*game).address.client_dll + *force_jump as u64;
    (*game).address.global_vars = (*game).address.client_dll + *global_vars as u64;

    if *DEBUG {
        println!("{} EntityList Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.entity_list as u32).bold());
        println!("{} ViewMatrix Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.matrix as u32).bold());
        println!("{} ViewAngle Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.view_angle as u32).bold());
        println!("{} LocalController Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.local_controller as u32).bold());
        println!("{} LocalPawn Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.local_pawn as u32).bold());
        println!("{} ForceJump Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.force_jump as u32).bold());
        println!("{} GlobalVars Address: {}", "[ INFO ]".blue().bold(), format!("{:X}", (*game).address.global_vars as u32).bold());
    }

    return (*game).address.client_dll != 0;
}

pub fn update_entity_list_entry() -> bool {
    let mut game = GAME.lock().unwrap();
    let mut entity_list_entry: u64 = 0;

    if !read_memory_auto((*game).address.entity_list, &mut entity_list_entry) {
        return false;
    }

    if !read_memory_auto(entity_list_entry + 0x10, &mut entity_list_entry) {
        return false;
    }

    (*game).address.entity_list_entry = entity_list_entry;
    return (*game).address.entity_list_entry != 0;
}

pub fn set_view_angle(yaw: f32, pitch: f32) -> bool {
    let game = GAME.lock().unwrap();
    let mut angle = Vector2 { x: pitch, y: yaw };

    if !write_memory_auto::<Vector2<f32>>((*game).address.view_angle, &mut angle) {
        return false;
    }

    return true;
}

pub fn set_force_jump(value: i32) -> bool {
    let game = GAME.lock().unwrap();
    let mut force_jump_value = value;

    if !write_memory_auto::<i32>((*game).address.force_jump, &mut force_jump_value) {
        return false;
    }

    return true;
}