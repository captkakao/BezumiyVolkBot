use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::Path;

type UserId = String;
type ChatId = String;
type Trigger = String;
type Reply = String;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub replies: HashMap<Trigger, Reply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Chat {
    pub name: String,
    pub users: HashMap<UserId, User>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DictionaryManager {
    pub chats: HashMap<ChatId, Chat>,
}

// Global instance as Option
pub(crate) static DICTIONARY: Mutex<Option<DictionaryManager>> = Mutex::new(None);

impl DictionaryManager {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(&self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;
        fs::write("dictionaries.json", data)
    }

    pub fn add_entry(&mut self, chat_id: ChatId, user_id: UserId, key: String, value: String) {
        let chat = self.chats.entry(chat_id).or_insert_with(|| Chat {
            name: "New Chat".to_string(),
            users: HashMap::new(),
        });

        let user = chat.users.entry(user_id).or_insert_with(|| User {
            username: "New User".to_string(),
            fullname: "New User".to_string(),
            replies: HashMap::new(),
        });

        user.replies.insert(key, value);
    }

    pub fn get_response(&self, chat_id: ChatId, user_id: UserId, key: &str) -> Option<&String> {
        let chat = self.chats.get(&chat_id)?;
        let user = chat.users.get(&user_id)?;
        user.replies.get(key)
    }
}

pub fn initialize_dictionary() -> Result<(), std::io::Error> {
    let manager = if Path::new("dictionaries.json").exists() {
        let data = fs::read_to_string("dictionaries.json")?;
        println!("Read data from file: {}", data);

        // Try to parse as Value first to see the raw structure
        let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();
        println!("Raw JSON structure: {:#?}", json_value);

        // Try to parse with explicit type annotations
        let parsed: DictionaryManager = match serde_json::from_str(&data) {
            Ok(m) => {
                println!("Successfully parsed dictionary");
                m
            }
            Err(e) => {
                println!("Failed to parse dictionary: {}", e);
                DictionaryManager::default()
            }
        };
        parsed
    } else {
        DictionaryManager::default()
    };


    println!("Manager before storing: {:#?}", manager);  // Debug print

    if let Ok(mut dict) = DICTIONARY.lock() {
        *dict = Some(manager);
        println!("Dictionary stored in static: {:#?}", dict);  // Debug print
    }
    Ok(())
}


// pub fn add_dictionary_entry(user_id: UserId, key: String, value: String) -> Result<(), std::io::Error> {
//     let mut lock = DICTIONARY.lock().map_err(|e| {
//         std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
//     })?;
// 
//     if let Some(manager) = lock.as_mut() {
//         manager.add_entry(user_id, key, value);
//         manager.save()?;
//     }
//     Ok(())
// }
// 
pub fn get_dictionary_response(chat_id: ChatId, user_id: UserId, key: &str) -> Option<String> {
    if let Ok(lock) = DICTIONARY.lock() {
        lock.as_ref()?.get_response(chat_id, user_id, key).cloned()
    } else {
        None
    }
}

// pub fn save_dictionary() -> Result<(), std::io::Error> {
//     let lock = DICTIONARY.lock().map_err(|e| {
//         std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
//     })?;
// 
//     if let Some(manager) = lock.as_ref() {
//         manager.save()?;
//     }
//     Ok(())
// }

pub fn print_dictionary() {
    if let Ok(lock) = DICTIONARY.lock() {
        if let Some(manager) = lock.as_ref() {
            println!("Dictionary contents:");
            
            let chats = &manager.chats;
            for (chat_id, chat) in chats {
                println!("Chat ID {}: Chat name {:#?}", chat_id, chat.name);
                for (user_id, user) in &chat.users {
                    println!("User ID {}: User name {:#?}", user_id, user.username);
                    for (trigger, reply) in &user.replies {
                        println!("Trigger {}: Reply {:#?}", trigger, reply);
                    }
                }
            }
        } else {
            println!("Dictionary is not initialized");
        }
    } else {
        println!("Failed to lock dictionary");
    }
}
