mod commands;
mod handlers;
mod utils;

use commands::Command;
use handlers::{start::*, help::*, init_users::*, add_trigger::*};
use teloxide::prelude::*;
use dotenv::dotenv;
use teloxide::sugar::request::RequestReplyExt;
use utils::dictionary::{get_dictionary_response, initialize_dictionary, print_dictionary};

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

                    if let Some(response) = get_dictionary_response(chat_id, username, text) {
                        bot.send_message(msg.chat.id, response)
                            .reply_to(msg)
                            .await?;
                        
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