use teloxide::{prelude::*, types::MessageId};
use crate::utils::dictionary::{add_user_trigger, add_common_trigger};
use std::time::Duration;

pub async fn add_trigger(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Err(e) = bot.delete_message(msg.chat.id, msg.id).await {
        println!("Failed to delete command message: {}", e);
    }

    if let Some(text) = msg.text() {
        let parts: Vec<&str> = text.splitn(3, ' ').collect();
        if parts.len() < 3 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /add tg_username trigger=reply").await?;
            return Ok(());
        }

        let tg_username = parts[1].trim_start_matches('@').to_string();
        let trigger = parts[2];
        let trigger_details: Vec<&str> = trigger.splitn(2, '=').collect();
        if trigger_details.len() < 2 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /add tg_username trigger=reply").await?;
            return Ok(());
        }

        let trigger_key = trigger_details[0].trim().to_lowercase();
        let trigger_value = trigger_details[1].trim().to_string();

        let chat_id = msg.chat.id.0.to_string();
        
        if tg_username == "all" {
            match add_common_trigger(chat_id, trigger_key.clone(), trigger_value.clone()) {
                Ok(_) => {
                    let success_msg = bot.send_message(
                        msg.chat.id,
                        format!("Added '{}' to your triggers dictionary!", trigger_key)
                    ).await?;
                    tokio::spawn(delete_message_after_delay(bot.clone(), success_msg.chat.id, success_msg.id, 1));

                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error adding trigger: {}", e)).await?;
                }
            }
        } else {
            match add_user_trigger(chat_id, tg_username, trigger_key.clone(), trigger_value.clone()) {
                Ok(_) => {
                    let success_msg = bot.send_message(
                        msg.chat.id,
                        format!("Added '{}' to your triggers dictionary!", trigger_key)
                    ).await?;
                    tokio::spawn(delete_message_after_delay(bot.clone(), success_msg.chat.id, success_msg.id, 1));

                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error adding trigger: {}", e)).await?;
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
