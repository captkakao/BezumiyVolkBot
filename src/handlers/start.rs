use teloxide::{prelude::*, types::Message};

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        "Hello! I'm your Rust Telegram bot. Use /help to see available commands."
    ).await?;
    Ok(())
}