// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::mem::size_of;
use std::ops::{BitAnd, Shl};
use std::time::Instant;

use mint::{Vector3, Vector2};

use crate::config::{ProgramConfig, Offsets};
use crate::ui::functions::{hotkey_index_to_io, distance_between_vec3};

use crate::utils::cheat::process::{rpm_auto, rpm_offset, trace_address};
use crate::cheat::classes::entity::CUtlVector;

pub fn is_enemy_at_crosshair(local_entity_pawn_address: u64, local_entity_controller_team_id: i32, game_address_entity_list: u64) -> (bool, bool, u64, Option<Vector3<f32>>) {
    let mut u_handle: u32 = 0;
    
    if !rpm_offset(local_entity_pawn_address, Offsets::C_CSPlayerPawnBase::m_iIDEntIndex as u64, &mut u_handle) {
        return (false, false, 0, None);
    }

    if !rpm_offset(local_entity_pawn_address, Offsets::C_CSPlayerPawnBase::m_iIDEntIndex as u64, &mut u_handle) {
        return (false, false, 0, None);
    }

    let list_entry: u64 = trace_address(game_address_entity_list, &[0x8 * u_handle.wrapping_shr(9) + 0x10, 0x0]);

    if list_entry == 0 {
        return (false, false, 0, None);
    }

    let mut pawn_address: u64 = 0;

    if let Some(sum) = (0x78 as u64).checked_mul(u_handle.bitand(0x1FF) as u64) {
        if !rpm_offset(list_entry, sum, &mut pawn_address) {
            return (false, false, 0, None);
        }
    }

    let mut entity_team_id = 0;
    let mut entity_health = 0;
    let mut entity_pos: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    if !rpm_offset(pawn_address, Offsets::C_BaseEntity::m_iTeamNum as u64, &mut entity_team_id) {
        return (false, false, 0, None);
    }

    if !rpm_offset(pawn_address, Offsets::C_BaseEntity::m_iHealth as u64, &mut entity_health) {
        return (false, false, 0, None);
    }

    if !rpm_offset(pawn_address, Offsets::C_BasePlayerPawn::m_vOldOrigin as u64, &mut entity_pos) {
        return (false, false, 0, None);
    }

    return (true, local_entity_controller_team_id != entity_team_id && entity_health > 0, pawn_address, Some(entity_pos));
}

pub fn is_enemy_visible(b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64) -> bool {
    return b_spotted_by_mask.bitand((1 as u64).shl(local_player_controller_index)) != 0 || local_b_spotted_by_mask.bitand((1 as u64).shl(i)) != 0;
}

pub fn get_bomb(bomb_address: u64) -> Option<u64> {
    let mut planted_bomb: u64 = 0;

    if !rpm_auto(bomb_address, &mut planted_bomb) {
        return None;
    }

    if !rpm_auto(planted_bomb, &mut planted_bomb) {
        return None;
    }

    return Some(planted_bomb);
}

pub fn get_bomb_planted(bomb_address: u64) -> bool {
    let mut is_bomb_planted: bool = false;

    if let Some(sum) = bomb_address.checked_sub(0x8) {
        if !rpm_auto(sum, &mut is_bomb_planted) {
            return false;
        }
    } else {
        return false;
    }

    return is_bomb_planted;
}

pub fn get_bomb_site(planted_bomb: u64) -> Option<String> {
    let mut site: u32 = 0;

    if !rpm_offset(planted_bomb, Offsets::C_PlantedC4::m_nBombSite as u64, &mut site) {
        return None;
    }

    if site == 1 {
        return Some("B".to_string());
    } else {
        return Some("A".to_string());
    }
}

pub fn get_bomb_position(planted_bomb: u64) -> Option<Vector3<f32>> {
    let mut bomb_node = 0;

    if !rpm_offset(planted_bomb, Offsets::C_BaseEntity::m_pGameSceneNode as u64, &mut bomb_node) {
        return None;
    }

    let mut bomb_pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    if !rpm_offset(bomb_node, Offsets::CGameSceneNode::m_vecAbsOrigin as u64, &mut bomb_pos) {
        return None;
    }

    return Some(bomb_pos);
}

#[derive(Copy, Clone, Debug)]
pub enum WeaponType {
    Pistol,
    Rifle,
    Submachine,
    Sniper,
    Shotgun,
    MachineGun,
    Equipment,
    Grenade,
    Bomb,
    Fists,
    Knife,
    Other,
    None
}

pub fn has_weapon(weapon_type: WeaponType) -> bool {
    return match weapon_type {
        WeaponType::Pistol => true,
        WeaponType::Rifle => true,
        WeaponType::Submachine => true,
        WeaponType::Sniper => true,
        WeaponType::Shotgun => true,
        WeaponType::MachineGun => true,
        _ => false
    };
}

pub fn parse_weapon(name: String) -> (WeaponType, &'static str) {
    return match name.as_str() {
        "ak47" => (WeaponType::Rifle, "AK-47"),
        "aug" => (WeaponType::Rifle, "AUG"),
        "awp" => (WeaponType::Sniper, "AWP"),
        "bizon" => (WeaponType::Submachine, "PP-Bizon"),
        "c4" => (WeaponType::Bomb, "Bomb"),
        "cz75a" => (WeaponType::Pistol, "CZ-75 Auto"),
        "deagle" => (WeaponType::Pistol, "Desert Eagle"),
        "decoy" => (WeaponType::Grenade, "Decoy Grenade"),
        "elite" => (WeaponType::Pistol, "Dual Berettas"),
        "fists" => (WeaponType::Fists, "Fists"),
        "famas" => (WeaponType::Rifle, "FAMAS"),
        "fiveseven" => (WeaponType::Pistol, "Five-SeveN"),
        "flashbang" => (WeaponType::Grenade, "Flashbang"),
        "g3sg1" => (WeaponType::Sniper, "G3SG1"),
        "galilar" => (WeaponType::Rifle, "Galil AR"),
        "glock" => (WeaponType::Pistol, "Glock"),
        "healthshot" => (WeaponType::Other, "MediShot"),
        "hkp2000" => (WeaponType::Pistol, "P2000"),
        "hegrenade" => (WeaponType::Grenade, "Grenade"),
        "incgrenade" => (WeaponType::Grenade, "Incendiary"),
        "knife" => (WeaponType::Knife, "Knife"),
        "m249" => (WeaponType::MachineGun, "M249"),
        "m4a1" => (WeaponType::Rifle, "M4A4"),
        "m4a1_silencer" => (WeaponType::Rifle, "M4A1-S"),
        "mac10" => (WeaponType::Submachine, "MAC-10"),
        "mag7" => (WeaponType::Shotgun, "MAG-7"),
        "molotov" => (WeaponType::Grenade, "Molotov"),
        "mp5sd" => (WeaponType::Submachine, "MP5-SD"),
        "mp7" => (WeaponType::Submachine, "MP7"),
        "mp9" => (WeaponType::Submachine, "MP9"),
        "negev" => (WeaponType::MachineGun, "Negev"),
        "nova" => (WeaponType::Shotgun, "Nova"),
        "p250" => (WeaponType::Pistol, "P250"),
        "p90" => (WeaponType::Submachine, "P90"),
        "revolver" => (WeaponType::Pistol, "Revolver"),
        "sawedoff" => (WeaponType::Shotgun, "Sawed-Off"),
        "scar20" => (WeaponType::Sniper, "SCAR-20"),
        "sg556" => (WeaponType::Rifle, "SG556"),
        "smokegrenade" => (WeaponType::Grenade, "Smoke"),
        "ssg08" => (WeaponType::Sniper, "SSG 08"),
        "taser" => (WeaponType::Equipment, "Zeus x27"),
        "tec9" => (WeaponType::Pistol, "TEC-9"),
        "ump45" => (WeaponType::Submachine, "UMP-45"),
        "usp_silencer" => (WeaponType::Pistol, "USP-S"),
        "xm1014" => (WeaponType::Shotgun, "XM1014"),
        _ => (WeaponType::Other, "")
    };
}

pub fn cache_to_punch(aim_punch_cache: CUtlVector) -> Option<Vector2<f32>> {
    let mut punch = Vector2 { x: 0.0, y: 0.0 };

    if aim_punch_cache.count <= 0 || aim_punch_cache.count > 0xFFFF {
        return None;
    }

    if !rpm_auto(aim_punch_cache.data + (aim_punch_cache.count - 1) * size_of::<Vector3<f32>>() as u64, &mut punch) {
        return None;
    }

    if punch.x == 0.0 && punch.y == 0.0 {
        return None;
    }

    return Some(punch);
}

pub fn is_io_pressed(key: usize) -> bool {
    match hotkey_index_to_io(key) {
        Ok(button) => {
            return button.is_pressed();
        },
        Err(key) => {
            return key.is_pressed();
        }
    }
}

pub fn is_feature_toggled(key: usize, mode: usize, toggle_toggled: &mut bool, toggle_changed: &mut Instant) -> bool {
    let pressed = is_io_pressed(key);

    if mode == 0 {
        return pressed;
    } else {
        if pressed && (*toggle_changed).elapsed() > ProgramConfig::CheatDelays::Toggle {
            *toggle_toggled = !*toggle_toggled;
            *toggle_changed = Instant::now();
        }

        return *toggle_toggled;
    }
}

pub fn calculate_distance(position: Vector3<f32>, local_position: Vector3<f32>) -> u32 {
    return distance_between_vec3(position, local_position) as u32 / 100;
}