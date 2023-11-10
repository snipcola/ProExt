use imgui::Ui;
use mint::{Vector3, Vector2};
use crate::{cheat::classes::bone::{BoneJointPos, bone_joint_list}, utils::config::Config, ui::main::color_u32_to_f32};

pub fn draw_bones(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], config: Config) {
    let mut previous: BoneJointPos = BoneJointPos { pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, screen_pos: Vector2 { x: 0.0, y: 0.0 }, is_visible: false };

    for bone_joint in bone_joint_list::LIST {
        previous.pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        
        for joint in bone_joint {
            let current = bone_pos_list[joint as usize];

            if (previous.pos == Vector3 { x: 0.0, y: 0.0, z: 0.0 }) {
                previous = current;
                continue;
            }

            if previous.is_visible && current.is_visible {
                ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_u32_to_f32(config.bone_color)).thickness(1.3).build();
            }

            previous = current;
        }
    }
}