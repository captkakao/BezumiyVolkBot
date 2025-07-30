use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "start the bot")]
    Start,
    #[command(description = "ping the bot")]
    Ping,
    #[command(description = "initialize users from chat")]
    InitUsers,
    #[command(description = "add new dictionary entry: /add username key=value or /add all key=value")]
    Add,
    #[command(description = "get dictionary entries: /getdict username or /getdict all", hide)]
    GetDict,
}