use serenity_self::builder::CreateCommand;
use serenity_self::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
