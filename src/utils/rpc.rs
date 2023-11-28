use std::{thread::{self, sleep}, time::{SystemTime, UNIX_EPOCH}};
use discord_rpc_client::Client;
use crate::utils::config::ProgramConfig;
use crate::utils::config::CONFIG;

pub fn set_rpc_activity(client: &mut Client, started: u64) {
    client
        .set_activity(| activity | {
            activity
                .state(ProgramConfig::Package::Description)
                .assets(| assets | assets.large_image(ProgramConfig::RPC::ImageAsset))
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
    if !ProgramConfig::RPC::Enabled {
        return;
    }

    thread::spawn(move || {
        let mut client = Client::new(ProgramConfig::RPC::ClientID);
        let started = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        client.start();
        
        loop {
            let config = CONFIG.lock().unwrap().clone();

            if config.settings.enabled && config.settings.discord_rpc_enabled {
                set_rpc_activity(&mut client, started);
            } else {
                clear_rpc_activity(&mut client);
            }

            // Delay
            sleep(ProgramConfig::ThreadDelays::RPC);
        }
    });
}