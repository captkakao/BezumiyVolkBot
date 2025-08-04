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
    #[command(description = "add new dictionary entry: \n/add tg_username trigger=reply or \n/add all trigger=reply")]
    Add,
    #[command(description = "delete dictionary entry: \n/delete tg_username trigger or \n/delete all trigger", hide)]
    Delete,
    #[command(description = "change trigger reply frequency: /changefrq 4")]
    ChangeFrq,
    #[command(description = "set roast level [1-5]: /setroastlvl 4")]
    SetRoastLvl,
    #[command(description = "get dictionary entries: /getdict", hide)]
    GetDict,
    #[command(description = "set dictionary entries: /setdict", hide)]
    SetDict,
}