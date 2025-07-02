use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponses {
    pub users: HashMap<String, HashMap<String, String>>
}

impl UserResponses {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read_to_string(path)?;
        let responses: UserResponses = serde_json::from_str(&file)?;
        Ok(responses)
    }

    // pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let json = serde_json::to_string_pretty(self)?;
    //     std::fs::write(path, json)?;
    //     Ok(())
    // }
}