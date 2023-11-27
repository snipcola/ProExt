use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use crate::utils::process_manager::{search_memory, get_process_module_handle, rpm_offset, rpm_auto};

lazy_static! {
    pub static ref ENTITY_LIST: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref MATRIX: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref VIEW_ANGLE: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref LOCAL_PLAYER_CONTROLLER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref LOCAL_PLAYER_PAWN: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref BOMB: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod Offsets {
    pub mod C_BaseEntity {  // CEntityInstance
        pub const m_iHealth: usize = 0x32C; // int32_t
        pub const m_iTeamNum: usize = 0x3BF; // uint8_t
        pub const m_pGameSceneNode: usize = 0x310; // CGameSceneNode*
        pub const m_fFlags: usize = 0x3C8; // uint32_t
    }
    
    pub mod CBasePlayerController { // C_BaseEntity
        pub const m_hPawn: usize = 0x60C; // CHandle<C_BasePlayerPawn>
        pub const m_iszPlayerName: usize = 0x640; // char[128]
    }
    
    pub mod CCSPlayerController { // CBasePlayerController
        pub const m_hPlayerPawn: usize = 0x7EC; // CHandle<C_CSPlayerPawn>
        pub const m_bPawnIsAlive: usize = 0x7F4; // bool
    }
    
    pub mod C_BasePlayerPawn { // C_BaseCombatCharacter
        pub const m_pObserverServices: usize = 0x10C0; // CPlayer_ObserverServices*
        pub const m_pCameraServices: usize = 0x10E0; // CPlayer_CameraServices*
        pub const m_vOldOrigin: usize = 0x1224; // Vector
    }

    pub mod C_CSPlayerPawnBase { // C_BasePlayerPawn
        pub const m_vecLastClipCameraPos: usize = 0x128C; // Vector
        pub const m_angEyeAngles: usize = 0x1510; // QAngle
        pub const m_pClippingWeapon: usize = 0x12A8; // C_CSWeaponBase*
        pub const m_iIDEntIndex: usize = 0x153C; // CEntityIndex
        pub const m_entitySpottedState: usize = 0x1630; // EntitySpottedState_t
        pub const m_ArmorValue: usize = 0x1508; // int32_t
    }

    pub mod CGameSceneNode {
        pub const m_vecAbsOrigin: usize = 0xC8; // Vector
    }

    pub mod CCSPlayerBase_CameraServices { // CPlayer_CameraServices
        pub const m_iFOVStart: usize = 0x214; // uint32_t
    }

    pub mod EntitySpottedState_t {
        use crate::cheat::classes::offsets::Offsets::C_CSPlayerPawnBase;
        
        pub const m_bSpottedByMask: usize = C_CSPlayerPawnBase::m_entitySpottedState + 0xC; // uint32_t[2]
    }

    pub mod CompositeMaterialEditorPoint_t {
        pub const m_vecCompositeMaterialAssemblyProcedures: usize = 0x1E0; // CUtlVector<CompositeMaterialAssemblyProcedure_t>
    }

    pub mod CPlayer_ObserverServices { // CPlayerPawnComponent
        pub const m_hObserverTarget: usize = 0x44; // CHandle<C_BaseEntity>
    }

    pub mod C_PlantedC4 { // CBaseAnimGraph
        pub const m_nBombSite: usize = 0xE84; // int32_t
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod Signatures {
    pub const dwEntityList: &str = "48 8B 0D ?? ?? ?? ?? 48 89 7C 24 ?? 8B FA C1";
    pub const dwLocalPlayerController: &str = "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 4F";
    pub const dwLocalPlayerPawn: &str = "48 8D 05 ?? ?? ?? ?? C3 CC CC CC CC CC CC CC CC 48 83 EC ?? 8B 0D";
    pub const dwPlantedC4: &str = "48 8B 15 ?? ?? ?? ?? FF C0 48 8D 4C 24";
    pub const dwViewAngles: &str = "48 8B 0D ?? ?? ?? ?? E9 ?? ?? ?? ?? CC CC CC CC 40 55";
    pub const dwViewMatrix: &str = "48 8D 0D ?? ?? ?? ?? 48 C1 E0 06";
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

    let client_dll = get_process_module_handle("client.dll") as u64;

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
            
            *view_angle = (address + 24896 - client_dll) as u32;
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