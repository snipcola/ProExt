// Copyright (c) 2024 Snipcola
// SPDX-License-Identifier: MIT

use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

use crate::config::Signatures;
use crate::utils::cheat::process::{search_memory, get_process_module_handle, rpm_offset, rpm_auto};

lazy_static! {
    pub static ref ENTITY_LIST: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref MATRIX: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref VIEW_ANGLE: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref LOCAL_PLAYER_CONTROLLER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref LOCAL_PLAYER_PAWN: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref BOMB: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

pub fn search_offsets(signature: &str, module_address: u64) -> Option<u64> {
    let address_list: Vec<u64> = search_memory(signature, module_address, module_address + 0x4000000, 1);
    let mut offsets: u32 = 0;

    if address_list.is_empty() {
        return None;
    }

    if !rpm_offset(address_list[0], 3, &mut offsets) {
        return None;
    }

    let return_item = match (address_list[0] + 7).checked_add(offsets as u64) {
        Some(value) => value,
        None => return None
    };

    if return_item != 0 {
        return Some(return_item);
    }

    return None;
}

pub fn update_offsets() -> Option<String> {
    let mut entity_list = ENTITY_LIST.lock().unwrap();
    let mut local_player_controller = LOCAL_PLAYER_CONTROLLER.lock().unwrap();
    let mut matrix = MATRIX.lock().unwrap();
    let mut view_angle = VIEW_ANGLE.lock().unwrap();
    let mut local_player_pawn = LOCAL_PLAYER_PAWN.lock().unwrap();
    let mut bomb = BOMB.lock().unwrap();

    let client_dll = get_process_module_handle("client.dll");

    if client_dll == 0 {
        return Some("ClientDLL".to_string());
    }

    match search_offsets(Signatures::dwEntityList, client_dll) {
        Some(address) => *entity_list = (address - client_dll) as u32,
        None => return Some("EntityList".to_string())
    };

    match search_offsets(Signatures::dwLocalPlayerController, client_dll) {
        Some(address) => *local_player_controller = (address - client_dll) as u32,
        None => return Some("LocalPlayerController".to_string())
    };

    match search_offsets(Signatures::dwViewMatrix, client_dll) {
        Some(address) => *matrix = (address - client_dll) as u32,
        None => return Some("ViewMatrix".to_string())
    };

    match search_offsets(Signatures::dwViewAngles, client_dll) {
        Some(mut address) => {
            if !rpm_auto(address, &mut address) {
                return Some("ViewAnglesMemory".to_string())
            };
            
            *view_angle = (address + 0x5390 - client_dll) as u32;
        },
        None => return Some("ViewAngles".to_string())
    };

    match search_offsets(Signatures::dwLocalPlayerPawn, client_dll) {
        Some(address) => *local_player_pawn = (address + 0x138 - client_dll) as u32,
        None => return Some("LocalPlayerPawn".to_string())
    };

    match search_offsets(Signatures::dwPlantedC4, client_dll) {
        Some(address) => *bomb = (address - client_dll) as u32,
        None => return Some("Bomb".to_string())
    };

    return None;
}