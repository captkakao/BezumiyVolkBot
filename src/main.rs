mod commands;
mod handlers;
mod utils;

use commands::Command;
use handlers::{start::*, help::*};
use teloxide::prelude::*;
use dotenv::dotenv;
use utils::dictionary::{add_dictionary_entry, get_dictionary_response, initialize_dictionary, print_dictionary};

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
                Command::Add => {
                    if let Some(text) = msg.text() {
                        let parts: Vec<&str> = text.splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            let entry: Vec<&str> = parts[1].split('=').collect();
                            if entry.len() == 2 {
                                let key = entry[0].trim().to_string();
                                let value = entry[1].trim().to_string();
                                let user_id = msg.from().map(|user| user.id.0.to_string()).unwrap_or_default();

                                match add_dictionary_entry(user_id, key.clone(), value.clone()) {
                                    Ok(_) => {
                                        bot.send_message(msg.chat.id, format!("Added '{}' to your dictionary!", key)).await?;
                                    }
                                    Err(e) => {
                                        bot.send_message(msg.chat.id, format!("Error adding entry: {}", e)).await?;
                                    }
                                }
                            } else {
                                bot.send_message(msg.chat.id, "Usage: /add key=value").await?;
                            }
                        }
                    }
                    Ok(())
                }
            }
        });

    // Handler for regular messages
    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(|bot: Bot, msg: Message| async move {
            if let Some(text) = msg.text() {
                if let Some(user_id) = msg.from().map(|user| user.id.0.to_string()) {
                    
                    println!("User {} says: {}", user_id, text);
                    
                    if let Some(response) = get_dictionary_response(user_id, text) {
                        bot.send_message(msg.chat.id, response).await?;
                        return Ok(());
                    }
                }

                let reply = format!("Fuck {}", text);
                bot.send_message(msg.chat.id, reply).await?;
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