use teloxide::{prelude::*, utils::command::BotCommands};
use crate::utils::dictionary::{add_trigger_dict, DICTIONARY};

pub async fn add_trigger(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        let parts: Vec<&str> = text.splitn(3, ' ').collect();
        if parts.len() < 3 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /add tg_username trigger=reply").await?;
            return Ok(());
        }

        let tg_username = parts[1].trim().to_string();
        let trigger = parts[2];
        let trigger_details: Vec<&str> = trigger.splitn(2, '=').collect();
        if trigger_details.len() < 2 {
            bot.send_message(msg.chat.id, "Invalid format. Usage: /add tg_username trigger=reply").await?;
            return Ok(());
        }

        let trigger_key = trigger_details[0].trim().to_string();
        let trigger_value = trigger_details[1].trim().to_string();

        let chat_id = msg.chat.id.0.to_string();

        match add_trigger_dict(chat_id, tg_username, trigger_key.clone(), trigger_value.clone()) {
            Ok(_) => {
                bot.send_message(msg.chat.id, format!("Added '{}' to your triggers dictionary!", trigger_key)).await?;
            }
            Err(e) => {
                bot.send_message(msg.chat.id, format!("Error adding trigger: {}", e)).await?;
            }
        }
    }

    Ok(())
}