use teloxide::prelude::*;
use crate::utils::dictionary::{default_roast_level, update_roast_level};

pub async fn set_roast_level(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(msg_text) = msg.text() {
        let parts: Vec<&str> = msg_text.splitn(2, ' ').collect();
        if parts.len() < 2 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /setroastlvl 4").await?;
            return Ok(());
        }

        let chat_id = msg.chat.id.0.to_string();
        let mut chat_roast_level: u8 = default_roast_level();
        if let Ok(msg_text) = parts[1].parse::<u8>() {
            chat_roast_level = msg_text;
        }
        
        if chat_roast_level < 1 || chat_roast_level > 5 {
            bot.send_message(msg.chat.id, "Invalid roast level. Valid range: 1-5").await?;
            return Ok(());
        }
        
        match update_roast_level(chat_id, chat_roast_level) {
            Ok(_) => {
                bot.send_message(msg.chat.id, "Roast level updated").await?;
            }
            Err(e) => {
                log::error!("Failed to update roast level: {}", e);
                bot.send_message(msg.chat.id, "Failed to update roast level").await?;
            }       
        }
    }
    
    Ok(())
}