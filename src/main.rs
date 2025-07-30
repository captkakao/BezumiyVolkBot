mod commands;
mod handlers;
mod utils;

use commands::Command;
use handlers::{start::*, help::*, init_users::*, add_trigger::*};
use dotenv::dotenv;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{
    prelude::*,
    update_listeners,
    dispatching::Dispatcher,
};
use utils::dictionary::{DICTIONARY, get_dictionary_response, initialize_dictionary, print_dictionary};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    // Initialize dictionary at startup
    if let Err(e) = initialize_dictionary() {
        log::error!("Failed to initialize dictionary: {}", e);
        return;
    }

    print_dictionary();

    let bot = Bot::from_env();

    match bot.delete_webhook().drop_pending_updates(true).await {
        Ok(_) => log::info!("Successfully cleared pending updates"),
        Err(e) => log::warn!("Failed to clear pending updates: {}", e),
    }


    // Handler for commands
    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(|bot: Bot, msg: Message, cmd: Command| async move {
            match cmd {
                Command::Help => help(bot, msg).await,
                Command::Start => start(bot, msg).await,
                Command::Ping => {
                    bot.send_message(msg.chat.id, "Pong !").await?;
                    Ok(())
                },
                Command::InitUsers => init_users(bot, msg).await,
                Command::Add => add_trigger(bot, msg).await,
            }
        });

    // Handler for regular messages
    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(|bot: Bot, msg: Message| async move {
            if let Some(text) = msg.text() {
                if let Some(user) = msg.from() {
                    let chat_id = msg.chat.id.0.to_string();
                    let username = user.username.clone().unwrap_or_default();

                    println!("User {} in chat {} says: {}", username, chat_id, text);

                    if let Some(response) = get_dictionary_response(chat_id.clone(), username, text.to_string()) {
                        let should_reply = if let Ok(mut lock) = DICTIONARY.lock() {
                            if let Some(manager) = lock.as_mut() {
                                let should_reply = manager.should_reply_to_message(&chat_id);
                                manager.save().ok();
                                should_reply
                            } else {
                                return Ok(());
                            }
                        } else {
                            return Ok(());
                        };

                        if should_reply {
                            bot.send_message(msg.chat.id, response)
                                .reply_to(msg)
                                .await?;
                        }

                        return Ok(());
                    }
                }
            }
            Ok(())
        }));

    // Combine both handlers
    let handler = dptree::entry()
        .branch(command_handler)
        .branch(message_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}