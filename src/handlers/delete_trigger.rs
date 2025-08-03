use teloxide::{prelude::*, types::MessageId};
use crate::utils::dictionary::{delete_user_trigger, delete_common_trigger};
use std::time::Duration;

pub async fn delete_trigger(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Err(e) = bot.delete_message(msg.chat.id, msg.id).await {
        println!("Failed to delete command message: {}", e);
    }

    if let Some(text) = msg.text() {
        let parts: Vec<&str> = text.splitn(3, ' ').collect();
        if parts.len() < 3 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /delete tg_username trigger").await?;
            return Ok(());
        }

        let tg_username = parts[1].trim_start_matches('@').to_string();
        let trigger = parts[2];

        let chat_id = msg.chat.id.0.to_string();
        
        if tg_username == "all" {
            match delete_common_trigger(chat_id, trigger.to_string()) {
                Ok(_) => {
                    let success_msg = bot.send_message(
                        msg.chat.id,
                        format!("Deleted '{}' from triggers dictionary!", trigger)
                    ).await?;
                    tokio::spawn(delete_message_after_delay(bot.clone(), success_msg.chat.id, success_msg.id, 1));

                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error deleting trigger: {}", e)).await?;
                }
            }
        } else {
            match delete_user_trigger(chat_id, tg_username, trigger.to_string()) {
                Ok(_) => {
                    let success_msg = bot.send_message(
                        msg.chat.id,
                        format!("Deleted '{}' from triggers dictionary!", trigger)
                    ).await?;
                    tokio::spawn(delete_message_after_delay(bot.clone(), success_msg.chat.id, success_msg.id, 1));

                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error deleting trigger: {}", e)).await?;
                }
            }
        }
    }

    Ok(())
}

async fn delete_message_after_delay(bot: Bot, chat_id: ChatId, message_id: MessageId, seconds: u64) {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    if let Err(e) = bot.delete_message(chat_id, message_id).await {
        println!("Failed to delete message: {}", e);
    }
}
