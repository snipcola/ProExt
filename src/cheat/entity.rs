use mint::{Vector2, Vector3};

use crate::utils::process_manager::{get_address_with_offset, read_memory, read_memory_auto, trace_address};
use crate::cheat::bone::Bone;
use crate::cheat::offsets::{ENTITY_OFFSETS, PAWN_OFFSETS};
use crate::cheat::game::GAME;

use super::bone::BoneJointPos;

pub struct CUtlVector {
    pub count: u64,
    pub data: u64
}

pub struct PlayerController {
    pub address: u64,
    pub team_id: i32,
    pub health: i32,
    pub alive_status: i32,
    pub pawn: u64,
    pub player_name: String
}

pub struct PlayerPawn {
    pub address: u64,
    pub bone_data: Bone,
    pub view_angle: Vector2<f32>,
    pub pos: Vector3<f32>,
    pub screen_pos: Vector2<f32>,
    pub camera_pos: Vector3<f32>,
    pub weapon_name: String,
    pub shots_fired: u64,
    pub aim_punch_angle: Vector2<f32>,
    pub aim_punch_cache: CUtlVector,
    pub health: i32,
    pub team_id: i32,
    pub fov: i32,
    pub b_spotted_by_mask: u64,
    pub f_flags: i32
}

pub enum Flags {
    None,
    InAir = 1 << 0
}

pub struct Entity {
    pub controller: PlayerController,
    pub pawn: PlayerPawn
}

impl Default for Entity {
    fn default() -> Self {
        return Entity {
            controller: PlayerController { address: 0, team_id: 0, health: 0, alive_status: 0, pawn: 0, player_name: "Name_None".to_string() },
            pawn: PlayerPawn { address: 0, bone_data: Bone { entity_pawn_address: 0, bone_pos_list: [BoneJointPos { pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, screen_pos: Vector2 { x: 0.0, y: 0.0 }, is_visible: false }; 30] }, view_angle: Vector2 { x: 0.0, y: 0.0 }, pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, screen_pos: Vector2 { x: 0.0, y: 0.0 }, camera_pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, weapon_name: "Weapon_None".to_string(), shots_fired: 0, aim_punch_angle: Vector2 { x: 0.0, y: 0.0 }, aim_punch_cache: CUtlVector { count: 0, data: 0 }, health: 0, team_id: 0, fov: 0, b_spotted_by_mask: 0, f_flags: 0 }
        }
    }
}

pub fn buffer_to_string(buffer: &[u8]) -> String {
    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    let cleaned_buffer = &buffer[..len];
    String::from_utf8_lossy(cleaned_buffer).to_string()
}

impl Entity {
    pub fn update_controller(&mut self, player_controller_address: u64) -> bool {
        if player_controller_address == 0 {
            return false;
        }

        self.controller.address = player_controller_address;

        if !self.controller.get_health() {
            return false;
        }

        if !self.controller.get_is_alive() {
            return false;
        }

        if !self.controller.get_team_id() {
            return false;
        }

        if !self.controller.get_player_name() {
            return false;
        }

        self.pawn.address = self.controller.get_player_pawn_address();
        return true;
    }

    pub fn update_pawn(&mut self, player_pawn_address: u64) -> bool {
        if player_pawn_address == 0 {
            return false;
        }

        self.pawn.address = player_pawn_address;

        if !self.pawn.get_camera_pos() {
            return false;
        }

        if !self.pawn.get_pos() {
            return false;
        }

        if !self.pawn.get_view_angle() {
            return false;
        }

        if !self.pawn.get_weapon_name() {
            return false;
        }

        if !self.pawn.get_aim_punch_angle() {
            return false;
        }

        if !self.pawn.get_shots_fired() {
            return false;
        }

        if !self.pawn.get_health() {
            return false;
        }

        if !self.pawn.get_team_id() {
            return false;
        }

        if !self.pawn.get_fov() {
            return false;
        }

        if !self.pawn.get_spotted() {
            return false;
        }

        if !self.pawn.get_f_flags() {
            return false;
        }

        if !self.pawn.get_aim_punch_cache() {
            return false;
        }

        if !self.pawn.bone_data.update_bone_data(player_pawn_address) {
            return false;
        }

        return true;
    }

    pub fn is_alive(&mut self) -> bool {
        return self.controller.alive_status == 1 && self.pawn.health > 0;
    }

    pub fn is_in_screen(&mut self) -> bool {
        return (*GAME.lock().unwrap()).view.world_to_screen(self.pawn.pos, &mut self.pawn.screen_pos);
    }

    pub fn get_bone(&mut self) -> Option<Bone> {
        if self.pawn.address == 0 {
            return None;
        }

        return Some(self.pawn.bone_data.clone());
    }
}

impl PlayerController {
    pub fn get_team_id(&mut self) -> bool {
        return get_address_with_offset(self.address, (*ENTITY_OFFSETS.lock().unwrap()).team_id, &mut self.team_id);
    }

    pub fn get_health(&mut self) -> bool {
        return get_address_with_offset(self.address, (*ENTITY_OFFSETS.lock().unwrap()).health, &mut self.health);
    }

    pub fn get_is_alive(&mut self) -> bool {
        return get_address_with_offset(self.address, (*ENTITY_OFFSETS.lock().unwrap()).is_alive, &mut self.alive_status);
    }

    pub fn get_player_name(&mut self) -> bool {
        let mut buffer: [u8; 260] = [0; 260];
        
        if !read_memory(self.address + (*ENTITY_OFFSETS.lock().unwrap()).is_player_name as u64, &mut buffer, 260) {
            return false;
        }

        self.player_name = buffer_to_string(&buffer);

        if self.player_name.is_empty() {
            self.player_name = "Name_None".to_string();
        }

        return true;
    }

    pub fn get_player_pawn_address(&mut self) -> u64 {
        let mut entity_pawn_list_entry = 0;
        let mut entity_pawn_address = 0;

        if !get_address_with_offset(self.address, (*ENTITY_OFFSETS.lock().unwrap()).player_pawn, &mut self.pawn) {
            return 0;
        }

        if !read_memory_auto((*GAME.lock().unwrap()).address.entity_list, &mut entity_pawn_list_entry) {
            return 0;
        }

        if !read_memory_auto(entity_pawn_list_entry + 0x10 + 8 * ((self.pawn & 0x7FFF) >> 9), &mut entity_pawn_list_entry) {
            return 0;
        }

        if !read_memory_auto(entity_pawn_list_entry + 0x78 * (self.pawn & 0x1FF), &mut entity_pawn_address) {
            return 0;
        }

        return entity_pawn_address;
    }
}

impl PlayerPawn {
    pub fn get_view_angle(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).ang_eye_angles, &mut self.view_angle);
    }

    pub fn get_camera_pos(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).vec_last_clip_camera_pos, &mut self.camera_pos);
    }

    pub fn get_spotted(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).b_spotted_by_mask, &mut self.b_spotted_by_mask);
    }

    pub fn get_weapon_name(&mut self) -> bool {
        let weapon_name_address = trace_address(self.address + (*PAWN_OFFSETS.lock().unwrap()).p_clipping_weapon as u64, &[0x10, 0x20, 0x0]);
        let mut buffer: [u8; 260] = [0; 260];

        if weapon_name_address == 0 {
            return false;
        }
        
        if !read_memory(weapon_name_address, &mut buffer, 260) {
            return false;
        }
        
        self.weapon_name = buffer_to_string(&buffer);

        if self.weapon_name.is_empty() {
            self.weapon_name = "Weapon_None".to_string();
        }

        return true;
    }

    pub fn get_shots_fired(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).i_shots_fired, &mut self.shots_fired);
    }

    pub fn get_aim_punch_angle(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).aim_punch_angle, &mut self.aim_punch_angle);
    }

    pub fn get_team_id(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).i_team_num, &mut self.team_id);
    }

    pub fn get_aim_punch_cache(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).aim_punch_cache, &mut self.aim_punch_cache);
    }

    pub fn get_pos(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).pos, &mut self.pos);
    }

    pub fn get_health(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).current_health, &mut self.health);
    }

    pub fn get_fov(&mut self) -> bool {
        let mut camera_services = 0;

        if !read_memory_auto(self.address + (*PAWN_OFFSETS.lock().unwrap()).camera_services as u64, &mut camera_services) {
            return false;
        }

        return get_address_with_offset(camera_services, (*PAWN_OFFSETS.lock().unwrap()).i_fov_start, &mut self.fov);
    }

    pub fn get_f_flags(&mut self) -> bool {
        return get_address_with_offset(self.address, (*PAWN_OFFSETS.lock().unwrap()).f_flags, &mut self.f_flags);
    }

    pub fn has_flag(&mut self, flag: Flags) -> bool {
        return self.f_flags & (flag as i32) != 0;
    }
}