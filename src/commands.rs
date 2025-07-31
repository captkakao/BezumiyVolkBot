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
    #[command(description = "add new dictionary entry: \n/add username key=value or \n/add all key=value")]
    Add,
    #[command(description = "change trigger reply frequency: /changefrq 4")]
    ChangeFrq,
    #[command(description = "get dictionary entries: /getdict username or /getdict all", hide)]
    GetDict,
}