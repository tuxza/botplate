use poise::serenity_prelude::CreateEmbedFooter;
use rand::prelude::IndexedRandom;

pub async fn make_numbers_pretty(num: u64) -> String {
    let s = num.to_string();
    let mut result = String::new();

    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    result.chars().rev().collect()
}

pub async fn random_footer() -> CreateEmbedFooter {
    let mut rng = rand::rng();
    let version = "v0.1.5"; // it would be a lot smarter to make this a constant but i only call it once so shut up
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
