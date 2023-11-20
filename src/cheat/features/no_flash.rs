use crate::{utils::process_manager::write_memory_auto, cheat::classes::offsets::Offsets};

pub fn run_no_flash(pawn_address: u64) {
    write_memory_auto(pawn_address + Offsets::C_CSPlayerPawnBase::m_flFlashDuration as u64, &mut 0.0);
}