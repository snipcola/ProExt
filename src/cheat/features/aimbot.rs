use std::{f32::consts::PI, sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::Ui;
use mint::{Vector3, Vector2};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use crate::{utils::{config::{Config, ProgramConfig, CONFIG}, mouse::{move_mouse, LAST_MOVED}}, ui::functions::{distance_between_vec2, color_with_masked_alpha, color_u32_to_f32}, cheat::{classes::{bone::{BoneIndex, BoneJointPos}, view::View}, functions::is_feature_toggled}};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().aimbot.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));

    pub static ref AB_LOCKED_ENTITY: Arc<Mutex<Option<(Instant, u64)>>> = Arc::new(Mutex::new(None));
    pub static ref AB_OFF_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

pub fn get_aimbot_toggled(config: Config) -> bool {
    let feature = config.aimbot;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn get_aimbot_yaw_pitch(config: Config, aim_pos: Vector3<f32>, camera_pos: Vector3<f32>, view_angle: Vector2<f32>) -> Option<f32> {
    let pos = Vector3 { x: aim_pos.x - camera_pos.x, y: aim_pos.y - camera_pos.y, z: aim_pos.z - camera_pos.z };
    let distance = (pos.x.powf(2.0) + pos.y.powf(2.0)).sqrt();

    let yaw = pos.y.atan2(pos.x) * 57.295779513 - view_angle.y;
    let pitch = -(pos.z / distance).atan() * 57.295779513 - view_angle.x;
    let norm = (yaw.powf(2.0) + pitch.powf(2.0)).sqrt() * 0.75;
    
    if norm > config.aimbot.fov {
        return None;
    }

    return Some(norm);
}

pub fn run_aimbot(config: Config, norm: f32, window_info: ((i32, i32), (i32, i32)), game_view: View, aim_pos: Vector3<f32>, address: u64) {
    let mut locked_entity = AB_LOCKED_ENTITY.lock().unwrap();

    if locked_entity.is_none() {
        *locked_entity = Some((Instant::now(), address));
    }
    
    if let Some((locked_on, entity_address)) = *locked_entity {
        if entity_address != address {
            *locked_entity = None;
            return;
        }

        let delay_offset = if config.aimbot.delay_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.aimbot.delay_offset as f32) .. config.aimbot.delay_offset as f32) * 1000.0).trunc() / 1000.0 };
        let delay = Duration::from_secs_f32((config.aimbot.delay as f32 + delay_offset).min(500.0).max(0.0) / 1000.0);
        
        if locked_on.elapsed() < delay {
            return;
        }
    }
    
    let base_smooth = 1.0;
    let smooth_offset = if config.aimbot.smooth_offset == 0.0 { 0.0 } else { (thread_rng().gen_range(-config.aimbot.smooth_offset .. config.aimbot.smooth_offset) * 1000.0).trunc() / 1000.0 };
    let smooth = (config.aimbot.smooth + smooth_offset).min(5.0).max(0.0) + base_smooth;
    
    let (screen_center_x, screen_center_y) = ((window_info.1.0 / 2) as f32, (window_info.1.1 / 2) as f32);
    let mut screen_pos = Vector2 { x: 0.0, y: 0.0 };

    if (*LAST_MOVED.lock().unwrap()).elapsed() < ProgramConfig::CheatDelays::Aimbot || !game_view.world_to_screen(aim_pos, &mut screen_pos, window_info) || ((screen_center_x - screen_pos.x).abs() <= 1.0 && (screen_center_y - screen_pos.y).abs() <= 1.0)  {
        return;
    }

    let mut target_x = if screen_pos.x > screen_center_x { -(screen_center_x - screen_pos.x) } else { screen_pos.x - screen_center_x };
    target_x /= smooth;
    target_x = if screen_pos.x > screen_center_x { if target_x + screen_center_x > screen_center_x * 2.0 { 0.0 } else { target_x } } else { if target_x + screen_center_x < 0.0 { 0.0 } else { target_x } };

    let mut target_y = if screen_pos.y > screen_center_y { -(screen_center_y - screen_pos.y) } else { screen_pos.y - screen_center_y };
    target_y /= smooth;
    target_y = if screen_pos.y > screen_center_y { if target_y + screen_center_y > screen_center_y * 2.0 { 0.0 } else { target_y } } else { if target_y + screen_center_y < 0.0 { 0.0 } else { target_y } };

    if smooth != base_smooth {
        target_x /= smooth * (base_smooth + (base_smooth - (norm / config.aimbot.fov)));
        target_y /= smooth * (base_smooth + (base_smooth - (norm / config.aimbot.fov)));

        target_x = if target_x.abs() < base_smooth { if target_x > 0.0 { base_smooth } else { -base_smooth } } else { target_x };
        target_y = if target_y.abs() < base_smooth { if target_y > 0.0 { base_smooth } else { -base_smooth } } else { target_y };
    }

    move_mouse(target_x as i32, target_y as i32, true);
}

pub fn get_aimbot_bone_indexes(config: Config) -> Vec<usize> {
    let mut bone_indexes = vec![];

    if config.aimbot.bone_head {
        bone_indexes.push(BoneIndex::Head as usize);
    }

    if config.aimbot.bone_neck {
        bone_indexes.push(BoneIndex::Neck0 as usize);
    }

    if config.aimbot.bone_spine {
        bone_indexes.push(BoneIndex::Spine1 as usize);
    }

    if config.aimbot.bone_pelvis {
        bone_indexes.push(BoneIndex::Pelvis as usize);
    }

    return bone_indexes;
}

pub fn aimbot_check(bone_pos_list: [BoneJointPos; 30], window_width: i32, window_height: i32, aim_pos: &mut Option<Vector3<f32>>, max_aim_distance: &mut f32, entity_address: &mut Option<u64>, address: u64, enemy_visible: bool, in_air: bool, config: Config) {
    if config.aimbot.only_grounded && in_air {
        return;
    }

    if config.aimbot.only_visible && !enemy_visible {
        return;
    }
    
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let bone_indexes = get_aimbot_bone_indexes(config);
    
    for bone_index in bone_indexes {
        let distance_to_sight = distance_between_vec2(bone_pos_list[bone_index].screen_pos, pos);

        if distance_to_sight < *max_aim_distance {
            *max_aim_distance = distance_to_sight;
            *entity_address = Some(address);
            *aim_pos = Some(bone_pos_list[bone_index].pos);

            if bone_index as usize == BoneIndex::Head as usize {
                if let Some(aim_pos) = aim_pos {
                    aim_pos.z -= -1.0;
                }
            }
        }
    }
}

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, aimbot_info: Option<f32>, config: Config) {
    let color = {
        if config.aimbot.fov_circle_target_enabled && aimbot_info.is_some() {
            config.aimbot.fov_circle_target_color
        } else {
            config.aimbot.fov_circle_color
        }
    };
    
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aimbot.fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    if config.aimbot.fov_circle_outline_enabled {
        ui.get_background_draw_list().add_circle(center_point, radius, color_with_masked_alpha(color, 0xFF000000)).thickness(config.aimbot.fov_circle_thickness + 1.0).build();
    }

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(color)).thickness(config.aimbot.fov_circle_thickness).build();
}