mod commands;
mod handlers;
mod utils;

use commands::Command;
use handlers::{
    start::*,
    help::*,
    init_users::*,
    add_trigger::*,
    delete_trigger::*,
    get_dict::*,
    set_dict::*,
    change_reply_frequency::*
};
use dotenv::dotenv;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{
    prelude::*,
    dispatching::Dispatcher,
};
use utils::dictionary::{DICTIONARY, get_dictionary_response, initialize_dictionary, print_dictionary};
use utils::deepseek::DeepSeekRoaster;

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

    let ai_roaster = match DeepSeekRoaster::new() {
        Ok(roaster) => {
            log::info!("DeepSeek AI roaster initialized successfully!");
            Some(roaster)
        },
        Err(e) => {
            log::warn!("DeepSeek AI roaster not available: {}. Using simple roasts as fallback.", e);
            None
        }
    };

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
                Command::Delete => delete_trigger(bot, msg).await,
                Command::GetDict => get_dict(bot, msg).await,
                Command::SetDict => set_dict(bot, msg).await,
                Command::ChangeFrq => change_reply_frequency(bot, msg).await,
            }
        });

    // Handler for regular messages
    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(move |bot: Bot, msg: Message| {
            let ai_roaster = ai_roaster.clone();
            async move {
                if let Some(text) = msg.text() {
                    if let Some(user) = msg.from() {
                        let chat_id = msg.chat.id.0.to_string();
                        let username = user.username.clone().unwrap_or_default();

                        if let Some(response) = get_dictionary_response(chat_id.clone(), username.clone(), text.to_string()) {
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

                        // let roast_chance = 1.0; // 100% chance to roast
                        // let roast_chance = 0.5; // 50% chance to roast
                        // let roast_chance = 0.2; // 20% chance to roast
                        let roast_chance = 0.15; // 15% chance to roast

                        if rand::random::<f32>() < roast_chance {
                            let roast = if let Some(roaster) = &ai_roaster {
                                match roaster.generate_roast(text, &username).await {
                                    Ok(ai_roast) => {
                                        log::info!("Generated AI roast for {}: {}", username, ai_roast);
                                        Some(ai_roast)
                                    }
                                    Err(e) => {
                                        log::warn!("AI roast failed: {}", e);
                                        None
                                    }
                                }
                            } else {
                                None
                            };

                            if let Some(roast_message) = roast {
                                bot.send_message(msg.chat.id, roast_message)
                                    .reply_to(msg)
                                    .await?;
                            }
                        }
                    }
                }
                Ok(())
            }
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