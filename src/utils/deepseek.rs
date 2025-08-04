
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
                    "content": "You are a savage, brutal roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate extremely rude, satirical, and harsh roasts that are brutally honest and cutting. Always respond in the SAME LANGUAGE as the user's message. If the message is in Kazakh, roast in Kazakh. If Russian, roast in Russian. If English, roast in English. You can and should use profanity, swear words, and harsh language appropriate to each language. Be merciless, sarcastic, and brutally satirical. Keep responses under 150 characters. DO NOT use any emojis in your responses. Be creative and savage in referencing the user's message content. Understand cultural context and use harsh humor for each language. NEVER use quotes around your response - respond directly without any formatting. Don't hold back - be as rude and offensive as possible while being clever."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 100,
            "temperature": 0.95,
            "top_p": 0.95
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
            let cleaned_content = content.trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim();
            Ok(cleaned_content.to_string())
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
                "kazakh" => "Бұл мүрдардың романы жазып жатыр",
                "russian" => "Этот долбоёб написал целый роман",
                _ => "This moron wrote a fucking novel"
            }
        } else if message_len < 5 {
            match language {
                "kazakh" => "Бұл ақылсыз ештеңе айта алмайды",
                "russian" => "Этот тупой даже слова связать не может",
                _ => "This idiot can barely string words together"
            }
        } else if is_caps {
            match language {
                "kazakh" => "Бұл дауыстап жазып жатыр, ақылын жоғалтты",
                "russian" => "Этот дебил орёт заглавными как психованный",
                _ => "This fucking lunatic is screaming in caps"
            }
        } else if has_emojis {
            match language {
                "kazakh" => "Бұл эмодзиге мас болған балапан",
                "russian" => "Этот долбоёб обожает эмодзи как малолетка",
                _ => "This emoji-obsessed manchild"
            }
        } else {
            match language {
                "kazakh" => "Бұл дурак айтты",
                "russian" => "Этот дебил сказал",
                _ => "This dumbass said"
            }
        };

        format!(
            "Brutally roast and destroy user {} who just said: {}. Context: {}. Be extremely rude, savage, and use profanity! Respond in the same language as the user's message. Do not use quotes around your response. Do not use any emojis. Be merciless and brutal!",
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