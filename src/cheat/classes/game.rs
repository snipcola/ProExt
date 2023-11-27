use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use crate::utils::process_manager::{get_process_module_handle, rpm_auto, rpm_offset};
use crate::cheat::classes::offsets::{ENTITY_LIST, MATRIX, VIEW_ANGLE, LOCAL_PLAYER_CONTROLLER, LOCAL_PLAYER_PAWN};
use crate::cheat::classes::view::View;
use crate::cheat::classes::offsets::BOMB;

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
            bomb: 0
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

#[derive(Clone)]
pub struct Game {
    pub address: Address,
    pub view: View
}

#[derive(Clone)]
pub struct Address {
    pub client_dll: u64,
    pub entity_list: u64,
    pub matrix: u64,
    pub view_angle: u64,
    pub entity_list_entry: u64,
    pub local_controller: u64,
    pub local_pawn: u64,
    pub bomb: u64
}

pub fn init_game_address() -> bool {
    let mut game = GAME.lock().unwrap();
    let entity_list = ENTITY_LIST.lock().unwrap();
    let matrix = MATRIX.lock().unwrap();
    let view_angle = VIEW_ANGLE.lock().unwrap();
    let local_player_controller = LOCAL_PLAYER_CONTROLLER.lock().unwrap();
    let local_player_pawn = LOCAL_PLAYER_PAWN.lock().unwrap();
    let bomb = BOMB.lock().unwrap();

    (*game).address.client_dll = get_process_module_handle("client.dll") as u64;
    (*game).address.entity_list = (*game).address.client_dll + *entity_list as u64;
    (*game).address.matrix = (*game).address.client_dll + *matrix as u64;
    (*game).address.view_angle = (*game).address.client_dll + *view_angle as u64;
    (*game).address.local_controller = (*game).address.client_dll + *local_player_controller as u64;
    (*game).address.local_pawn = (*game).address.client_dll + *local_player_pawn as u64;
    (*game).address.bomb = (*game).address.client_dll + *bomb as u64;

    return (*game).address.client_dll != 0;
}

pub fn update_entity_list_entry() -> bool {
    let mut game = GAME.lock().unwrap();
    let mut entity_list_entry: u64 = 0;

    if !rpm_auto((*game).address.entity_list, &mut entity_list_entry) {
        return false;
    }

    if !rpm_offset(entity_list_entry, 0x10, &mut entity_list_entry) {
        return false;
    }

    (*game).address.entity_list_entry = entity_list_entry;
    return (*game).address.entity_list_entry != 0;
}