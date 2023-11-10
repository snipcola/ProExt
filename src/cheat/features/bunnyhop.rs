use crate::cheat::classes::game::set_force_jump;

pub fn run_bunny_hop(toggled: bool, has_flag_in_air: bool) {
    if toggled && has_flag_in_air {
        set_force_jump(65537);
    } else if toggled && !has_flag_in_air {
        set_force_jump(256);
        set_force_jump(65537);
        set_force_jump(256);
    } else {
        set_force_jump(256);
    }
}