#![allow(non_snake_case, non_upper_case_globals)]

pub mod Offsets {
    pub mod client_dll {
        pub const dwEntityList: usize = 0x1A146E8;
        pub const dwLocalPlayerController: usize = 0x1A64E80;
        pub const dwLocalPlayerPawn: usize = 0x1868CC8;
        pub const dwPlantedC4: usize = 0x1A84F90;
        pub const dwViewAngles: usize = 0x1A89710;
        pub const dwViewMatrix: usize = 0x1A7F610;
    }

    pub mod C_BaseEntity {
        pub const m_iHealth: usize = 0x344; // int32
        pub const m_iTeamNum: usize = 0x3E3; // uint8
        pub const m_pGameSceneNode: usize = 0x328; // CGameSceneNode*
        pub const m_fFlags: usize = 0x3EC; // uint32
        pub const m_nSubclassID: usize = 0x378; // CUtlStringToken
    }
    
    pub mod CBasePlayerController {
        pub const m_hPawn: usize = 0x62C; // CHandle<C_BasePlayerPawn>
        pub const m_iszPlayerName: usize = 0x660; // char[128]
    }
    
    pub mod CCSPlayerController {
        pub const m_hPlayerPawn: usize = 0x80C; // CHandle<C_CSPlayerPawn>
        pub const m_bPawnIsAlive: usize = 0x814; // bool
    }
    
    pub mod C_BasePlayerPawn {
        pub const m_pObserverServices: usize = 0x11C0; // CPlayer_ObserverServices*
        pub const m_pCameraServices: usize = 0x11E0; // CPlayer_CameraServices*
        pub const m_vOldOrigin: usize = 0x1324; // Vector
    }

    pub mod C_CSPlayerPawnBase {
        pub const m_vecLastClipCameraPos: usize = 0x1384; // Vector
        pub const m_angEyeAngles: usize = 0x1438; // QAngle
        pub const m_pClippingWeapon: usize = 0x13A0; // C_CSWeaponBase*
        pub const m_iIDEntIndex: usize = 0x1458; // CEntityIndex
        pub const m_entitySpottedState: usize = 0x23D0; // EntitySpottedState_t
        pub const m_ArmorValue: usize = 0x241C; // int32
        pub const m_iShotsFired: usize = 0x23FC; // int32
    }

    pub mod C_CSPlayerPawn {
        pub const m_aimPunchCache: usize = 0x15A8; // CUtlVector<QAngle>
    }

    pub mod CGameSceneNode {
        pub const m_vecAbsOrigin: usize = 0xD0; // Vector
    }

    pub mod CCSPlayerBase_CameraServices {
        pub const m_iFOVStart: usize = 0x214; // uint32
    }

    pub mod EntitySpottedState_t {        
        pub const m_bSpottedByMask: usize = 0xC; // uint32[2]
    }

    pub mod CSkeletonInstance {
        pub const m_modelState: usize = 0x170; // CModelState
    }

    pub mod CPlayer_ObserverServices {
        pub const m_hObserverTarget: usize = 0x44; // CHandle<C_BaseEntity>
    }

    pub mod C_PlantedC4 {
        pub const m_nBombSite: usize = 0xF94; // int32
    }

    pub mod CBasePlayerWeaponVData {
        pub const m_iMaxClip1: usize = 0x2F4; // int32
    }

    pub mod C_BasePlayerWeapon {
        pub const m_iClip1: usize = 0x1678; // int32
    }
}

pub mod ProgramConfig {
    pub mod Package {
        pub const Name: &str = "ProExt";
        pub const Description: &str = "An open-source, external CS2 cheat.";
        pub const Executable: &str = "proext.exe";
        pub const Version: &str = env!("CARGO_PKG_VERSION");
        pub const Authors: &str = &env!("CARGO_PKG_AUTHORS");
    }

    pub mod Imgui {
        pub const FontSize: f32 = 13.0;

        pub mod FontPaths {
            pub const Chinese: &str = "C:/Windows/Fonts/msyh.ttc";
            pub const Cryillic: &str = "C:/Windows/Fonts/Arial.ttf";
            pub const Arabic: &str = "C:/Windows/Fonts/calibri.ttf";
        }
    }

    pub mod Update {
        pub const Enabled: bool = true;
        pub const URL: &str = "https://git.snipcola.com/snipcola/ProExt/raw/branch/main/bin/proext.exe";
        pub const CargoTomlURL: &str = "https://git.snipcola.com/snipcola/ProExt/raw/branch/main/Cargo.toml";
    }

    pub mod Links {
        pub const Source: &str = "https://git.snipcola.com/snipcola/ProExt";
        pub const License: &str = "https://git.snipcola.com/snipcola/ProExt/raw/branch/main/LICENSE";
    }

    pub mod Keys {
        use glutin::event::VirtualKeyCode;
        use mki::Keyboard;

        pub const Available: [&str; 20] = ["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Side Mouse", "Extra Mouse", "Shift", "Control", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];

        pub const ToggleKey: VirtualKeyCode = VirtualKeyCode::Insert;
        pub const ToggleKeyMKI: Keyboard = Keyboard::Insert;

        pub const ExitKey: VirtualKeyCode = VirtualKeyCode::End;
        pub const ExitKeyMKI: Keyboard = Keyboard::Other(0x23);
    }

    pub mod TargetProcess {
        pub const Executable: &str = "cs2.exe";
        pub const MaxAttempts: u32 = 30;
        pub const InitAddressesMaxAttempts: u32 = 15;

        pub mod Window {
            pub const Title: &str = "Counter-Strike 2";
            pub const Class: &str = "SDL_app";
        }
    }

    pub mod CheckDelays {
        use std::time::Duration;

        pub const AttachProcess: Duration = Duration::from_millis(1000);
        pub const InitAddresses: Duration = Duration::from_millis(1000);
    }

    pub mod ThreadDelays {
        use std::time::Duration;
        
        pub const UpdateConfigs: Duration = Duration::from_millis(250);
        pub const WindowTasks: Duration = Duration::from_millis(25);
        pub const IOTasks: Duration = Duration::from_millis(25);
    }

    pub mod CheatDelays {
        use std::time::Duration;

        pub const Toggle: Duration = Duration::from_millis(200);
        pub const Aimbot: Duration = Duration::from_millis(10);
        pub const AimbotOffEntity: Duration = Duration::from_millis(500);
        pub const TriggerbotOffEntity: Duration = Duration::from_millis(500);
    }
}