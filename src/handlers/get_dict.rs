use std::fs;
use teloxide::prelude::*;
use teloxide::types::InputFile;

pub async fn get_dict(bot: Bot, msg: Message) -> ResponseResult<()> {
    let file_path = "/app/dictionaries.json";

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let input_file = InputFile::memory(content.into_bytes())
                .file_name("dictionaries.json");

            bot.send_document(msg.chat.id, input_file).await?;
        }
        Err(e) => {
            log::error!("Failed to read dictionaries.json: {}", e);
            bot.send_message(msg.chat.id, "Failed to read dictionaries file").await?;
        }
    }
    Ok(())
}