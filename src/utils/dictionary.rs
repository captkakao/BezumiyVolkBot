use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::Path;

type Username = String;
type ChatId = String;
type Trigger = String;
type Reply = String;

pub(crate) fn default_reply_frequency() -> u32 { 2 }

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub fullname: String,
    pub replies: HashMap<Trigger, Reply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Chat {
    pub message_counter: u32,
    
    #[serde(default = "default_reply_frequency")]
    pub reply_frequency: u32,
    
    pub name: String,
    pub users: HashMap<Username, User>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DictionaryManager {
    pub chats: HashMap<ChatId, Chat>,
}

// Global instance as Option
pub(crate) static DICTIONARY: Mutex<Option<DictionaryManager>> = Mutex::new(None);

impl DictionaryManager {
    pub fn should_reply_to_message(&mut self, chat_id: &ChatId) -> bool {
        let chat = self.chats.entry(chat_id.clone()).or_insert_with(|| Chat {
            name: "New Chat".to_string(),
            users: HashMap::new(),
            message_counter: 0,
            reply_frequency: default_reply_frequency(),
        });

        if chat.reply_frequency == 0 {
            chat.reply_frequency = default_reply_frequency();
        }

        chat.message_counter += 1;
        chat.message_counter % chat.reply_frequency == 0
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(&self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;
        fs::write("dictionaries.json", data)
    }

    pub fn add_entry(&mut self, chat_id: ChatId, username: Username, trigger: String, reply: String) {
        let chat = self.chats.entry(chat_id).or_insert_with(|| Chat {
            message_counter: 0,
            reply_frequency: default_reply_frequency(),
            name: "New Chat".to_string(),
            users: HashMap::new(),
        });
        
        if chat.reply_frequency == 0 {
            chat.reply_frequency = default_reply_frequency()
        }

        let user = chat.users.entry(username).or_insert_with(|| User {
            fullname: "New User".to_string(),
            replies: HashMap::new(),
        });

        user.replies.insert(trigger, reply);
    }

    pub fn get_response(&self, chat_id: ChatId, username: Username, key: String) -> Option<&String> {
        let chat = self.chats.get(&chat_id)?;
        let user = chat.users.get(&username)?;

        let lowercase_input = key.to_lowercase();

        user.replies
            .iter()
            .find(|(k, _)| lowercase_input.contains(&k.to_lowercase()))
            .map(|(_, v)| v)
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


pub fn add_trigger_dict(chat_id: ChatId, username: Username, trigger: String, reply: String) -> Result<(), std::io::Error> {
    let mut lock = DICTIONARY.lock().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    if let Some(manager) = lock.as_mut() {
        manager.add_entry(chat_id, username, trigger, reply);
        manager.save()?;
    }
    Ok(())
}

pub fn get_dictionary_response(chat_id: ChatId, username: Username, key: String) -> Option<String> {
    if let Ok(lock) = DICTIONARY.lock() {
        lock.as_ref()?.get_response(chat_id, username, key).cloned()
    } else {
        None
    }
}

pub fn print_dictionary() {
    if let Ok(lock) = DICTIONARY.lock() {
        if let Some(manager) = lock.as_ref() {
            println!("Dictionary contents:");
            
            let chats = &manager.chats;
            for (chat_id, chat) in chats {
                println!("Chat ID {}: Chat name {:#?}", chat_id, chat.name);
                for (user_id, user) in &chat.users {
                    println!("User ID {}: User full name {:#?}", user_id, user.fullname);
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
