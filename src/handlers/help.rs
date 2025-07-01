use teloxide::{prelude::*, types::Message};
use teloxide::utils::command::BotCommands;
use crate::commands::Command;

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        Command::descriptions().to_string()
    ).await?;
    Ok(())
}