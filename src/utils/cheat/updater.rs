use ureq::get;
use toml::{Value, de::from_str};

use semver::Version;
use crate::config::ProgramConfig;

fn get_latest_version() -> Option<String> {
    let mut response = match get(ProgramConfig::Update::CargoTomlURL).call() {
        Ok(response) => response,
        Err(_) => return None
    };

    let response_string = match response.body_mut().read_to_string() {
        Ok(text) => text,
        Err(_) => return None
    };

    let parsed_toml: Value = match from_str::<Value>(&response_string) {
        Ok(value) => value,
        Err(_) => return None,
    };

    return parsed_toml
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|version| version.as_str().map(|s| s.to_string()));
}

fn update_exists() -> bool {
    match get(ProgramConfig::Update::URL).call() {
        Ok(response) => response.status() == 200,
        Err(_) => false
    }
}

pub fn update_available() -> Option<String> {
    if !update_exists() {
        return None;
    }

    let current_version = ProgramConfig::Package::Version;
    let latest_version = get_latest_version()?;

    let current = Version::parse(current_version).ok()?;
    let latest = Version::parse(&latest_version).ok()?;

    return match latest > current {
        true => Some(latest_version),
        false => None
    };
}
