use crate::errors::Error;

const RULES: &[&str] = &[
    "1. Be respectful to everyone — no harassment, bullying, or hate speech.",
    "2. Keep content appropriate for all ages — no NSFW, 18+, or gore content.",
    "3. No illegal activity — this includes doxxing, sharing malicious links, exploiting, or impersonating others.",
    "4. Don't share personal information (yours or others'), including addresses, passwords, or private accounts.",
    "5. Keep arguments civil — prolonged drama, baiting, or instigation is not allowed. Do not bring outside drama into the server.",
    "6. Respect the decisions of moderators and admins. If you disagree, discuss it calmly in private.",
    "7. Don't advertise other servers, services, or social media without admin permission.",
    "8. Follow channel-specific rules and post in the correct channels.",
    "9. No spamming, excessive pinging, disruptive behavior, or self-promotion/scamming.",
    "10. No evading punishments — alternate accounts used to bypass bans or mutes will result in removal.",
    "11. Keep usernames, nicknames, profile pictures, and statuses appropriate.",
    "12. Do not abuse bots, commands, or loopholes in botplate. Please report these as a bug.",
    "13. All members must comply with Discord's official Terms of Service, Privacy Policy, and Community Guidelines.",
    "14. Staff may take action on behavior not explicitly listed if it harms the community.",
    "15. Rules may be updated at any time — continued participation means acceptance of changes.",
];

pub async fn rulez(number: u8) -> Result<&'static str, Error> {
    RULES
        .get((number as usize).wrapping_sub(1))
        .copied()
        .ok_or_else(|| {
            Error::Custom(format!(
                "there's no rule #{number}! we only have {} rules.",
                RULES.len()
            ))
        })
}
