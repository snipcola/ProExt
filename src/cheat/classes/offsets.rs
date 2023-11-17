use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};

use crate::utils::process_manager::{search_memory, read_memory_auto, get_process_module_handle};

lazy_static! {
    pub static ref ENTITY_LIST: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref MATRIX: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref VIEW_ANGLE: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref LOCAL_PLAYER_CONTROLLER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref LOCAL_PLAYER_PAWN: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref BOMB: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref FORCE_JUMP: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));
    pub static ref GLOBAL_VARS: Arc<Mutex<u32>> = Arc::new(Mutex::new(0x0));

    pub static ref ENTITY_OFFSETS: Arc<Mutex<EntityOffsets>> = Arc::new(Mutex::new(EntityOffsets {
        health: 0x32C,
        team_id: 0x3BF,
        is_alive: 0x7F4,
        player_pawn: 0x60C,
        is_player_name: 0x640
    }));
    
    pub static ref PAWN_OFFSETS: Arc<Mutex<PawnOffsets>> = Arc::new(Mutex::new(PawnOffsets {
        pos: 0x1224,
        max_health: 0x328,
        current_health: 0x32C,
        game_scene_node: 0x310,
        bone_array: 0x1E0,
        ang_eye_angles: 0x1510,
        vec_last_clip_camera_pos: 0x128C,
        vec_abs_origin: 0xC8,
        clipping_weapon: 0x12A8,
        flash_duration: 0x1468,
        ent_index: 0x153C,
        team_num: 0x3BF,
        camera_services: 0x10E0,
        fov_start: 0x214,
        flags: 0x3C8,
        spotted_by_mask: 0x1630 + 0xC,
        observer_services: 0x10C0
    }));
    
    pub static ref GLOBAL_VAR_OFFSETS: Arc<Mutex<GlobalVarOffsets>> = Arc::new(Mutex::new(GlobalVarOffsets {
        real_time: 0x00,
        frame_count: 0x04,
        max_clients: 0x10,
        interval_per_tick: 0x14,
        current_time: 0x2C,
        current_time2: 0x30,
        tick_count: 0x40,
        interval_per_tick2: 0x44,
        current_netchan: 0x0048,
        current_map: 0x0180,
        current_map_name: 0x0188
    }));

    pub static ref BOMB_OFFSETS: Arc<Mutex<BombOffsets>> = Arc::new(Mutex::new(BombOffsets {
        bomb_site: 0xE84
    }));
    
    pub static ref SIGNATURES: Arc<Mutex<Signatures>> = Arc::new(Mutex::new(Signatures {
        global_vars: "48 89 0D ?? ?? ?? ?? 48 89 41".to_string(),
        view_matrix: "48 8D 0D ?? ?? ?? ?? 48 C1 E0 06".to_string(),
        view_angles: "48 8B 0D ?? ?? ?? ?? E9 ?? ?? ?? ?? CC CC CC CC 40 55".to_string(),
        entity_list: "48 8B 0D ?? ?? ?? ?? 48 89 7C 24 ?? 8B FA C1".to_string(),
        local_player_controller: "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 4F".to_string(),
        force_jump: "48 8B 05 ?? ?? ?? ?? 48 8D 1D ?? ?? ?? ?? 48 89 45".to_string(),
        local_player_pawn: "48 8D 05 ?? ?? ?? ?? C3 CC CC CC CC CC CC CC CC 48 83 EC ?? 8B 0D".to_string(),
        bomb: "48 8B 15 ?? ?? ?? ?? FF C0 48 8D 4C 24".to_string()
    }));
}

pub struct EntityOffsets {
    pub health: u32,
    pub team_id: u32,
    pub is_alive: u32,
    pub player_pawn: u32,
    pub is_player_name: u32,
}

pub struct PawnOffsets {
    pub pos: u32,
    pub max_health: u32,
    pub current_health: u32,
    pub game_scene_node: u32,
    pub bone_array: u32,
    pub ang_eye_angles: u32,
    pub vec_last_clip_camera_pos: u32,
    pub vec_abs_origin: u32,
    pub clipping_weapon: u32,
    pub flash_duration: u32,
    pub ent_index: u32,
    pub team_num: u32,
    pub camera_services: u32,
    pub fov_start: u32,
    pub flags: u32,
    pub spotted_by_mask: u32,
    pub observer_services: u32
}

pub struct GlobalVarOffsets {
    pub real_time: u32,
    pub frame_count: u32,
    pub max_clients: u32,
    pub interval_per_tick: u32,
    pub current_time: u32,
    pub current_time2: u32,
    pub tick_count: u32,
    pub interval_per_tick2: u32,
    pub current_netchan: u32,
    pub current_map: u32,
    pub current_map_name: u32
}

pub struct BombOffsets {
    pub bomb_site: u32
}

pub struct Signatures {
    pub global_vars: String,
    pub view_matrix: String,
    pub view_angles: String,
    pub entity_list: String,
    pub local_player_controller: String,
    pub force_jump: String,
    pub local_player_pawn: String,
    pub bomb: String
}

pub fn search_offsets(signature: String, module_address: u64) -> Option<u64> {
    let address_list: Vec<u64> = search_memory(&signature, module_address, module_address + 0x4000000, 1);
    let mut offsets: u32 = 0;

    if address_list.is_empty() {
        return None;
    }

    if !read_memory_auto(address_list[0] + 3, &mut offsets) {
        return None;
    }

    let return_item = address_list[0] + offsets as u64 + 7;

    if return_item != 0 {
        return Some(return_item);
    }

    return None;
}

pub fn update_offsets() -> Option<String> {
    let signatures = SIGNATURES.lock().unwrap();
    let mut entity_list = ENTITY_LIST.lock().unwrap();
    let mut local_player_controller = LOCAL_PLAYER_CONTROLLER.lock().unwrap();
    let mut matrix = MATRIX.lock().unwrap();
    let mut global_vars = GLOBAL_VARS.lock().unwrap();
    let mut view_angle = VIEW_ANGLE.lock().unwrap();
    let mut local_player_pawn = LOCAL_PLAYER_PAWN.lock().unwrap();
    let mut force_jump = FORCE_JUMP.lock().unwrap();
    let mut bomb = BOMB.lock().unwrap();

    let client_dll = get_process_module_handle("client.dll") as u64;
    if client_dll == 0 { return Some("ClientDLL".to_string()); }

    match search_offsets(signatures.entity_list.clone(), client_dll) {
        Some(address) => *entity_list = (address - client_dll) as u32,
        _ => { return Some("EntityList".to_string()) }
    };

    match search_offsets(signatures.local_player_controller.clone(), client_dll) {
        Some(address) => *local_player_controller = (address - client_dll) as u32,
        _ => { return Some("LocalPlayerController".to_string()) }
    };

    match search_offsets(signatures.view_matrix.clone(), client_dll) {
        Some(address) => *matrix = (address - client_dll) as u32,
        _ => { return Some("ViewMatrix".to_string()) }
    };

    match search_offsets(signatures.global_vars.clone(), client_dll) {
        Some(address) => *global_vars = (address - client_dll) as u32,
        _ => { return Some("GlobalVars".to_string()) }
    };

    match search_offsets(signatures.view_angles.clone(), client_dll) {
        Some(mut address) => {
            if !read_memory_auto(address, &mut address) { return Some("ViewAnglesMemory".to_string()) };
            *view_angle = (address + 24896 - client_dll) as u32;
        },
        _ => { return Some("ViewAngles".to_string()) }
    };

    match search_offsets(signatures.local_player_pawn.clone(), client_dll) {
        Some(address) => *local_player_pawn = (address + 0x138 - client_dll) as u32,
        _ => { return Some("LocalPlayerPawn".to_string()) }
    };

    match search_offsets(signatures.force_jump.clone(), client_dll) {
        Some(address) => *force_jump = (address + 0x30 - client_dll) as u32,
        _ => { return Some("ForceJump".to_string()) }
    };

    match search_offsets(signatures.bomb.clone(), client_dll) {
        Some(address) => *bomb = (address - client_dll) as u32,
        _ => { return Some("Bomb".to_string()) }
    };

    return None;
}