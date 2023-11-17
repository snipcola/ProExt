use std::{sync::{Arc, Mutex}, thread, time::{SystemTime, UNIX_EPOCH}};
use discord_rpc_client::Client;
use lazy_static::lazy_static;
use crate::utils::config::RPC_CLIENT_ID;
use crate::utils::config::CONFIG;

lazy_static! {
    pub static ref RPC_CLIENT: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new(*RPC_CLIENT_ID)));
}

fn current_time_u64() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    
    return since_epoch.as_secs();
}

pub fn set_rpc_activity(client: &mut Client) {
    client
        .set_activity(| activity | {
            activity
                .state("An open-source, external CS2 cheat.")
                .assets(| assets | assets.large_image("cs2"))
                .timestamps(| timestamps | timestamps.start(current_time_u64()))
        })
        .ok();
}

pub fn clear_rpc_activity(client: &mut Client) {
    client
        .clear_activity()
        .ok();
}

pub fn initialize_rpc() {
    let rpc_client = RPC_CLIENT.clone();
    let mut rpc_enabled = false;
    
    thread::spawn(move || {
        let mut client = rpc_client.lock().unwrap();
        client.start();
        
        loop {
            let config = CONFIG.lock().unwrap().clone();

            if config.misc.enabled && config.misc.discord_rpc_enabled {
                if !rpc_enabled {
                    set_rpc_activity(&mut client);
                    
                    rpc_enabled = true;
                    (*CONFIG.lock().unwrap()).misc.discord_rpc_enabled = true;
                }
            } else {
                if rpc_enabled {
                    clear_rpc_activity(&mut client);
                    
                    rpc_enabled = false;
                    (*CONFIG.lock().unwrap()).misc.discord_rpc_enabled = false;
                }
            }
        }
    });
}