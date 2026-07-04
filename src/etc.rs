use poise::serenity_prelude::CreateEmbedFooter;
use rand::prelude::IndexedRandom;
// use std::time::Instant;
use sysinfo::System;

pub struct SysInfo {
    pub os_name: Option<String>,
    pub os_vers: Option<String>,
    pub h_used_memory: u64,
    pub h_total_memory: u64,
    pub h_uptime: u64,
    pub bot_memory: u64,
}

pub async fn fn_that_does_nothing() -> u64 {
    0
}

pub async fn get_sysinfo() -> SysInfo {
    let mut sys = System::new();
    sys.refresh_all();

    let pid = sysinfo::get_current_pid().expect("fuck you, failed to get PID for some reason");

    let bot_memory = sys.process(pid).map(|p| p.memory()).unwrap_or(0);

    // h = host if you cant read

    SysInfo {
        os_name: System::name(),
        os_vers: System::os_version(),
        h_used_memory: sys.used_memory(),
        h_total_memory: sys.total_memory(),
        h_uptime: System::uptime(),
        bot_memory,
    }
}

pub async fn random_footer() -> CreateEmbedFooter {
    let mut rng = rand::rng();
    let version = "v0.1.0"; // it would be a lot smarter to make this a constant but i only call it once so shut up
    let messages = [
        "botplate-rs is cool",
        "come in here dear boy have a cigar your gonna go far",
        "check out our github repo!",
        "how random is random..?",
        "tuxzilla is in your walls",
        "yo yall seen those creepy footers??",
        "FOOTER",
        "dude why are you reading this",
        "duckie please stop dming me",
        "billions of tuxaroos",
        "billions must love",
        "wait what is this server again",
    ];
    let Some(message) = messages.choose(&mut rng) else {
        return CreateEmbedFooter::new(format!("botplate-rs reimagined | {}", version));
    };
    CreateEmbedFooter::new(format!(
        "{} | botplate-rs reimagined | {}",
        message, version
    ))
}

pub async fn convert_bytes_2_gigabytes(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
}

pub async fn convert_bytes_2_megabytes(bytes: u64) -> String {
    format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
}

pub async fn convert_uptime_2_human(uptime: u64) -> String {
    let seconds = uptime % 60;
    let minutes = (uptime / 60) % 60;
    let hours = (uptime / 3600) % 24;
    let days = uptime / 86400;
    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}
