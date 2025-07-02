mod commands;
mod handlers;

use commands::Command;
use handlers::{start::*, help::*};
use teloxide::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

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
                }
            }
        });

    // Handler for regular messages
    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(|bot: Bot, msg: Message| async move {
            if let Some(text) = msg.text() {
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