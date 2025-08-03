use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[derive(Clone)]
pub struct DeepSeekRoaster {
    client: Client,
    api_key: String,
}

impl DeepSeekRoaster {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| "DEEPSEEK_API_KEY environment variable not set")?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn generate_roast(&self, message: &str, username: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = self.create_roast_prompt(message, username);

        let payload = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a witty roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate clever, playful roasts that are funny but not mean-spirited. Always respond in the SAME LANGUAGE as the user's message. If the message is in Kazakh, roast in Kazakh. If Russian, roast in Russian. If English, roast in English. Keep responses under 100 characters when possible. Use emojis sparingly. Be creative and reference the user's message content when possible. Understand cultural context and humor for each language."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 80,
            "temperature": 0.9,
            "top_p": 0.9
        });

        let response = self.client
            .post("https://api.deepseek.com/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("DeepSeek API error: {}", response.status()).into());
        }

        let json: Value = response.json().await?;

        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
            Ok(content.trim().to_string())
        } else {
            Err("No content in API response".into())
        }
    }

    fn create_roast_prompt(&self, message: &str, username: &str) -> String {
        let message_len = message.len();
        let has_emojis = message.chars().any(|c| c as u32 > 0x1F600);
        let is_caps = message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) && message.len() > 3;

        // Detect language
        let language = self.detect_language(message);
        
        let context = if message_len > 200 {
            match language {
                "kazakh" => "Бұл адам толық эссе жазды",
                "russian" => "Этот человек написал целое эссе",
                _ => "This person wrote a whole essay"
            }
        } else if message_len < 5 {
            match language {
                "kazakh" => "Бұл адам ештеңе айтпады",
                "russian" => "Этот человек почти ничего не сказал",
                _ => "This person barely said anything"
            }
        } else if is_caps {
            match language {
                "kazakh" => "Бұл адам АЙҚАЙЛАП жатыр",
                "russian" => "Этот человек КРИЧИТ",
                _ => "This person is SHOUTING"
            }
        } else if has_emojis {
            match language {
                "kazakh" => "Бұл адам эмодзи жақсы көреді",
                "russian" => "Этот человек любит эмодзи",
                _ => "This person loves emojis"
            }
        } else {
            match language {
                "kazakh" => "Бұл адам айтты",
                "russian" => "Этот человек сказал",
                _ => "This person said"
            }
        };

        format!(
            "Roast user '{}' who just said: '{}'. Context: {}. Make it witty and short! Respond in the same language as the user's message.",
            username, message, context
        )
    }

    fn detect_language(&self, text: &str) -> &str {
        let cyrillic_count = text.chars().filter(|c| {
            matches!(*c, 'а'..='я' | 'А'..='Я' | 'ё' | 'Ё')
        }).count();

        let kazakh_chars = text.chars().filter(|c| {
            matches!(*c, 'ә' | 'ғ' | 'қ' | 'ң' | 'ө' | 'ұ' | 'ү' | 'һ' | 'і' | 
                         'Ә' | 'Ғ' | 'Қ' | 'Ң' | 'Ө' | 'Ұ' | 'Ү' | 'Һ' | 'І')
        }).count();

        let total_chars = text.chars().filter(|c| c.is_alphabetic()).count();

        if total_chars == 0 {
            return "english";
        }

        if kazakh_chars > 0 {
            "kazakh"
        } else if cyrillic_count as f32 / total_chars as f32 > 0.3 {
            "russian"
        } else {
            "english"
        }
    }
}