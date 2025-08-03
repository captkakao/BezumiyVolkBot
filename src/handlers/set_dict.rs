use std::fs;
use std::env;
use std::io::Cursor;
use teloxide::net::Download;
use teloxide::prelude::*;
use crate::utils::dictionary::initialize_dictionary;

pub async fn set_dict(bot: Bot, msg: Message) -> ResponseResult<()> {
    let file_path = if env::var("APP_ENV").unwrap() == "test" {
        "./dictionaries.json"
    } else {
        "/app/dictionaries.json"
    };

    if let Some(document) = msg.document() {
        if let Some(file_name) = &document.file_name {
            if !file_name.ends_with(".json") {
                bot.send_message(msg.chat.id, "Please upload a JSON file").await?;
                return Ok(());
            }
        }

        let document_id = document.file.id.clone();

        let file = bot.get_file(document_id).await?;

        let mut file_content = Vec::new();
        let mut cursor = Cursor::new(&mut file_content);
        bot.download_file(&file.path, &mut cursor).await?;

        match std::str::from_utf8(&file_content) {
            Ok(json_str) => {
                if let Err(e) = serde_json::from_str::<serde_json::Value>(json_str) {
                    log::error!("Invalid JSON format: {}", e);
                    bot.send_message(msg.chat.id, "Invalid JSON format").await?;
                    return Ok(());
                }

                match fs::write(file_path, json_str) {
                    Ok(_) => {
                        log::info!("Dictionary file updated successfully");

                        if let Err(e) = initialize_dictionary() {
                            log::error!("Failed to initialize dictionary: {}", e);
                        }
                        
                        bot.send_message(msg.chat.id, "Dictionary file updated successfully!").await?;
                    }
                    Err(e) => {
                        log::error!("Failed to write dictionary file: {}", e);
                        bot.send_message(msg.chat.id, "Failed to update dictionary file").await?;
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to parse file content as UTF-8: {}", e);
                bot.send_message(msg.chat.id, "File content is not valid UTF-8").await?;
            }
        }
    } else {
        bot.send_message(msg.chat.id, "Please attach a JSON file to update the dictionary").await?;
    }

    Ok(())
}