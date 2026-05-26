use super::*;

#[test]
fn test_intent_creation() {
    let intents = Intents::new();
    assert_eq!(intents.bits(), 0);

    let intents = Intents::all();
    assert!(intents.guilds());
    assert!(intents.public_guild_messages());
}

#[test]
fn test_intent_operations() {
    let mut intents = Intents::none();
    assert!(!intents.guilds());

    intents = intents.with_guilds();
    assert!(intents.guilds());

    let other = Intents::none().with_public_guild_messages();
    let combined = intents | other;
    assert!(combined.guilds());
    assert!(combined.public_guild_messages());
}

#[test]
fn test_privileged_intents() {
    let intents = Intents::none().with_guild_messages();
    assert!(intents.has_privileged());

    let intents = Intents::none().with_forums();
    assert!(intents.has_privileged());

    let intents = Intents::none().with_public_guild_messages();
    assert!(!intents.has_privileged());
}

#[test]
fn test_display() {
    let intents = Intents::none();
    assert_eq!(format!("{}", intents), "Intents(NONE)");

    let intents = Intents::none().with_guilds().with_public_guild_messages();
    let display = format!("{}", intents);
    assert!(display.contains("GUILDS"));
    assert!(display.contains("PUBLIC_GUILD_MESSAGES"));
}
