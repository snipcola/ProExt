use crate::{utils::process_manager::write_memory_auto, cheat::classes::offsets::PAWN_OFFSETS};

pub fn run_no_flash(pawn_address: u64) {
    write_memory_auto(pawn_address + (*PAWN_OFFSETS.lock().unwrap()).fl_flash_duration as u64, &mut 0.0);
}