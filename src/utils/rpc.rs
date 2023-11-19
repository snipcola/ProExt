use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::{SystemTime, UNIX_EPOCH}};
use discord_rpc_client::Client;
use lazy_static::lazy_static;
use crate::utils::config::{RPC_CLIENT_ID, RPC_STATE, RPC_IMAGE_ASSET};
use crate::utils::config::CONFIG;
use crate::utils::config::THREAD_DELAYS;

lazy_static! {
    pub static ref RPC_CLIENT: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new(*RPC_CLIENT_ID)));
}

pub fn set_rpc_activity(client: &mut Client, started: u64) {
    client
        .set_activity(| activity | {
            activity
                .state(&*RPC_STATE)
                .assets(| assets | assets.large_image(&*RPC_IMAGE_ASSET))
                .timestamps(| timestamps | timestamps.start(started))
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
    let started = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    thread::spawn(move || {
        let mut client = rpc_client.lock().unwrap();
        client.start();
        
        loop {
            let config = CONFIG.lock().unwrap().clone();

            if config.misc.enabled && config.misc.discord_rpc_enabled {
                set_rpc_activity(&mut client, started);
            } else {
                clear_rpc_activity(&mut client);
            }

            sleep(THREAD_DELAYS.rpc);
        }
    });
}