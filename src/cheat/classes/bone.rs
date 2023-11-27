use mint::{Vector3, Vector2};

use crate::utils::process_manager::{rpm_offset, rpm_auto};
use crate::cheat::classes::offsets::Offsets;
use crate::cheat::classes::view::View;

#[derive(Clone, Copy)]
pub enum BoneIndex {
    Head = 6,
    Neck0 = 5,
    Spine1 = 4,
    Spine2 = 2,
    Pelvis = 0,
    ArmUpperL = 8,
    ArmLowerL = 9,
    HandL = 10,
    ArmUpperR = 13,
    ArmLowerR = 14,
    HandR = 15,
    LegUpperL = 22,
    LegLowerL = 23,
    AnkleL = 24,
    LegUpperR = 25,
    LegLowerR = 26,
    AnkleR = 27
}

pub fn aim_position_to_bone_index(aim_position: usize) -> usize {
    if aim_position == 1 {
        return BoneIndex::Neck0 as usize;
    } else if aim_position == 2 {
        return BoneIndex::Spine1 as usize;
    } else {
        return BoneIndex::Head as usize;
    }
}

#[derive(Clone, Copy)]
pub struct BoneJointData {
    pub pos: Vector3<f32>,
    pub pad: [u8; 0x14]
}

#[derive(Clone, Copy)]
pub struct BoneJointPos {
    pub pos: Vector3<f32>,
    pub screen_pos: Vector2<f32>,
    pub is_visible: bool
}

#[derive(Clone)]
pub struct Bone {
    pub entity_pawn_address: u64,
    pub bone_pos_list: [BoneJointPos; 30]
}

impl Bone {
    pub fn update_bone_data(&mut self, entity_pawn_address: u64, window_info: ((i32, i32), (i32, i32)), view: View) -> bool {
        if entity_pawn_address == 0 {
            return false;
        }

        self.entity_pawn_address = entity_pawn_address;

        let mut game_scene_node: u64 = 0;
        let mut bone_array_address: u64 = 0;

        if !rpm_offset(entity_pawn_address, Offsets::C_BaseEntity::m_pGameSceneNode as u64, &mut game_scene_node) {
            return false;
        }

        if !rpm_offset(game_scene_node, Offsets::CompositeMaterialEditorPoint_t::m_vecCompositeMaterialAssemblyProcedures  as u64, &mut bone_array_address) {
            return false;
        }

        let mut bone_array: [BoneJointData; 30] = [BoneJointData { pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, pad: [0; 0x14] }; 30];

        if !rpm_auto(bone_array_address, &mut bone_array) {
            return false;
        }

        for i in 0 .. 30 {
            let mut screen_pos = Vector2 { x: 0.0, y: 0.0 };
            let mut is_visible = false;

            if view.world_to_screen(bone_array[i].pos, &mut screen_pos, window_info) {
                is_visible = true;
            }

            self.bone_pos_list[i] = BoneJointPos { pos: bone_array[i].pos, screen_pos, is_visible };
        }

        return self.bone_pos_list.len() > 0;
    }
}

pub mod bone_joint_list {
    use crate::cheat::classes::bone::BoneIndex;

    pub static TRUNK: [BoneIndex; 4] = [BoneIndex::Head, BoneIndex::Neck0, BoneIndex::Spine2, BoneIndex::Pelvis];
    pub static LEFT_ARM: [BoneIndex; 4] = [BoneIndex::Neck0, BoneIndex::ArmUpperL, BoneIndex::ArmLowerL, BoneIndex::HandL];
    pub static RIGHT_ARM: [BoneIndex; 4] = [BoneIndex::Neck0, BoneIndex::ArmUpperR, BoneIndex::ArmLowerR, BoneIndex::HandR];
    pub static LEFT_LEG: [BoneIndex; 4] = [BoneIndex::Pelvis, BoneIndex::LegUpperL, BoneIndex::LegLowerL, BoneIndex::AnkleL];
    pub static RIGHT_LEG: [BoneIndex; 4] = [BoneIndex::Pelvis, BoneIndex::LegUpperR, BoneIndex::LegLowerR, BoneIndex::AnkleR];
    pub static LIST: [[BoneIndex; 4]; 5] = [TRUNK, LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG];
}