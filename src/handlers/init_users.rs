use teloxide::{prelude::*, types::{Message, ChatMemberKind}};
use teloxide::utils::command::BotCommands;
use std::collections::HashMap;
use crate::{
    commands::Command,
    utils::dictionary::{DICTIONARY, Chat, User}
};

pub async fn init_users(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0.to_string();
    let chat_title = msg.chat.title().unwrap_or("Unknown Chat").to_string();

    let bot_member = bot.get_chat_member(msg.chat.id, bot.get_me().await?.id).await?;

    match bot_member.kind {
        ChatMemberKind::Administrator(admin) => {
            if !admin.can_delete_messages {
                bot.send_message(
                    msg.chat.id,
                    "I need 'Delete Messages' permission to work properly.\n\
                     Please update my admin permissions to include:\n\
                     - Delete Messages\n\
                     - Send Messages\n\
                     - Read Messages"
                ).await?;
            }
        }
        _ => {
            bot.send_message(
                msg.chat.id,
                "I need to be an administrator to work properly.\n\
                 Please add me as admin with these permissions:\n\
                 - Delete Messages\n\
                 - Send Messages\n\
                 - Read Messages"
            ).await?;
            return Ok(());
        }
    }

    // First, get the admins
    let admins = bot.get_chat_administrators(msg.chat.id).await?;

    // Prepare the data outside of the mutex lock
    let users_data: Vec<_> = admins.into_iter().map(|member| {
        let user = member.user;
        let username = user.username.unwrap_or_default();
        let fullname = format!("{} {}",
                               user.first_name,
                               user.last_name.unwrap_or_default()
        ).trim().to_string();

        (username, fullname)
    }).collect();

    // Update dictionary in a separate scope to ensure MutexGuard is dropped
    let update_result = {
        let mut lock = DICTIONARY.lock().map_err(|e| {
            teloxide::RequestError::Api(teloxide::ApiError::Unknown(e.to_string()))
        })?;

        if let Some(manager) = lock.as_mut() {
            // Get or create chat entry
            let chat = manager.chats.entry(chat_id.clone()).or_insert_with(|| {
                Chat {
                    message_counter: 0,
                    reply_frequency: crate::utils::dictionary::default_reply_frequency(),
                    name: chat_title.clone(),
                    users: HashMap::new(),
                }
            });

            // Update chat name
            chat.name = chat_title;

            if chat.reply_frequency == 0 {
                chat.reply_frequency = crate::utils::dictionary::default_reply_frequency();
            }

            // Add or update members
            for (username, fullname) in users_data {
                if !username.is_empty() {  // Only add users with usernames
                    let user_entry = chat.users.entry(username).or_insert_with(|| {
                        User {
                            fullname: fullname.clone(),
                            replies: HashMap::new(),
                        }
                    });
                }
            }

            let users_count = chat.users.len();
            manager.save().map(|_| users_count)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Dictionary not initialized"
            ))
        }
    };

    // Handle results after the mutex is dropped
    match update_result {
        Ok(users_count) => {
            bot.send_message(
                msg.chat.id,
                format!("Successfully initialized {} users", users_count)
            ).await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("Failed to update dictionary: {}", e)
            ).await?;
        }
    }

    Ok(())
}