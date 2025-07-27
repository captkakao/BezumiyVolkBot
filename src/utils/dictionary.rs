use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::Path;

// Type aliases for cleaner code
type UserId = String;
type Dictionary = HashMap<String, String>;
type UserDictionaries = HashMap<UserId, Dictionary>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DictionaryManager {
    #[serde(flatten)]  // This will flatten the structure to match your JSON
    dictionaries: UserDictionaries,
}


// Global instance as Option
static DICTIONARY: Mutex<Option<DictionaryManager>> = Mutex::new(None);

impl DictionaryManager {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(&self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;
        fs::write("dictionaries.json", data)
    }

    pub fn add_entry(&mut self, user_id: UserId, key: String, value: String) {
        self.dictionaries
            .entry(user_id)
            .or_insert_with(HashMap::new)
            .insert(key, value);
    }

    pub fn get_response(&self, user_id: UserId, key: &str) -> Option<&String> {
        self.dictionaries.get(&user_id)?.get(key)
    }

    pub fn remove_entry(&mut self, user_id: UserId, key: &str) -> bool {
        if let Some(dict) = self.dictionaries.get_mut(&user_id) {
            dict.remove(key).is_some()
        } else {
            false
        }
    }

    pub fn get_all_entries(&self, user_id: UserId) -> Option<&Dictionary> {
        self.dictionaries.get(&user_id)
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


pub fn add_dictionary_entry(user_id: UserId, key: String, value: String) -> Result<(), std::io::Error> {
    let mut lock = DICTIONARY.lock().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    if let Some(manager) = lock.as_mut() {
        manager.add_entry(user_id, key, value);
        manager.save()?;
    }
    Ok(())
}

pub fn get_dictionary_response(user_id: UserId, key: &str) -> Option<String> {
    if let Ok(lock) = DICTIONARY.lock() {
        lock.as_ref()?.get_response(user_id, key).cloned()
    } else {
        None
    }
}


pub fn remove_dictionary_entry(user_id: UserId, key: &str) -> Result<bool, std::io::Error> {
    let mut lock = DICTIONARY.lock().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    if let Some(manager) = lock.as_mut() {
        let removed = manager.remove_entry(user_id, key);
        if removed {
            manager.save()?;
        }
        Ok(removed)
    } else {
        Ok(false)
    }
}

pub fn save_dictionary() -> Result<(), std::io::Error> {
    let lock = DICTIONARY.lock().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    if let Some(manager) = lock.as_ref() {
        manager.save()?;
    }
    Ok(())
}

pub fn print_dictionary() {
    if let Ok(lock) = DICTIONARY.lock() {
        if let Some(manager) = lock.as_ref() {
            println!("Dictionary contents:");
            for (user_id, dict) in &manager.dictionaries {
                println!("User {}: {:#?}", user_id, dict);
            }
        } else {
            println!("Dictionary is not initialized");
        }
    } else {
        println!("Failed to lock dictionary");
    }
}
