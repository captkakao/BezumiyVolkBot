use teloxide::prelude::*;
use crate::utils::dictionary::{default_reply_frequency, update_reply_frequency};

pub async fn change_reply_frequency(bot: Bot, msg: Message) -> ResponseResult<()> {

    if let Some(msg_text) = msg.text() {
        let parts: Vec<&str> = msg_text.splitn(2, ' ').collect();
        if parts.len() < 2 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /changefrq 4").await?;
            return Ok(());
        }

        let chat_id = msg.chat.id.0.to_string();
        let mut reply_frq: u32 = default_reply_frequency();
        if let Ok(msg_text) = parts[1].parse::<u32>() {
            reply_frq = msg_text;
        }
        
        match update_reply_frequency(chat_id, reply_frq) {
            Ok(_) => {
                bot.send_message(msg.chat.id, "Reply frequency updated").await?;
            }
            Err(e) => {
                log::error!("Failed to update reply frequency: {}", e);
                bot.send_message(msg.chat.id, "Failed to update reply frequency").await?;
            }       
        }
    }
    
    Ok(())
}