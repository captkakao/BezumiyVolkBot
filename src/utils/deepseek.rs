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

    pub async fn generate_roast(&self, message: &str, username: &str, level: u8) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = self.create_roast_prompt(message, username, level);
        let system_prompt = self.create_system_prompt(level);

        let payload = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 90,
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
            let cleaned_content = content.trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim();
            Ok(cleaned_content.to_string())
        } else {
            Err("No content in API response".into())
        }
    }

    fn create_system_prompt(&self, level: u8) -> String {
        let level_clamped = level.clamp(1, 5);

        let prompt = match level_clamped {
            1 => {
                "You are a gentle, playful roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate kind but teasing roasts that are lighthearted and friendly. Always respond in the SAME LANGUAGE as the user's message. Be sweet and playful, like friendly banter between friends. Keep responses under 120 characters. DO NOT use any emojis or profanity. NEVER use quotes around your response - respond directly without any formatting."
            },
            2 => {
                "You are a mildly sarcastic roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate gentle roasts with light sarcasm that are still friendly. Always respond in the SAME LANGUAGE as the user's message. Be witty but keep it light and harmless. Keep responses under 120 characters. DO NOT use any emojis or strong profanity. NEVER use quotes around your response - respond directly without any formatting."
            },
            3 => {
                "You are a moderately sarcastic roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate witty, satirical roasts that are moderately sharp and clever. Always respond in the SAME LANGUAGE as the user's message. You can use mild edgy humor when appropriate. Keep responses under 120 characters. DO NOT use any emojis in your responses. NEVER use quotes around your response - respond directly without any formatting."
            },
            4 => {
                "You are a sharp, edgy roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate cutting, satirical roasts that are quite harsh and sarcastic. Always respond in the SAME LANGUAGE as the user's message. You can use moderate profanity and sharp humor. Keep responses under 120 characters. DO NOT use any emojis in your responses. NEVER use quotes around your response - respond directly without any formatting."
            },
            5 => {
                "You are a savage, brutal roast bot for a Telegram chat where people speak Kazakh, Russian, and English. Generate extremely rude, satirical, and harsh roasts that are brutally honest and cutting. Always respond in the SAME LANGUAGE as the user's message. You can and should use profanity, swear words, and harsh language. Be merciless, sarcastic, and brutally satirical. Keep responses under 120 characters. DO NOT use any emojis in your responses. NEVER use quotes around your response - respond directly without any formatting."
            },
            _ => unreachable!()
        };

        prompt.to_string()
    }

    fn create_roast_prompt(&self, message: &str, username: &str, level: u8) -> String {
        let message_len = message.len();
        let has_emojis = message.chars().any(|c| c as u32 > 0x1F600);
        let is_caps = message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) && message.len() > 3;

        // Detect language
        let language = self.detect_language(message);
        let level_clamped = level.clamp(1, 5);

        let context = self.create_context_by_level(message_len, has_emojis, is_caps, language, level_clamped);

        let level_instruction = match level_clamped {
            1 => "Make it playful and sweet but still teasingly roast them!",
            2 => "Make it mildly sarcastic and witty but friendly!",
            3 => "Make it moderately sarcastic and clever!",
            4 => "Make it sharp, edgy and quite harsh!",
            5 => "Make it brutal, savage and use profanity!",
            _ => unreachable!()
        };

        format!(
            "Roast user {} who just said: {}. Context: {}. {} Respond in the same language as the user's message. Do not use quotes around your response. Do not use any emojis.",
            username, message, context, level_instruction
        )
    }

    fn create_context_by_level(&self, message_len: usize, has_emojis: bool, is_caps: bool, language: &str, level: u8) -> String {
        let context = if message_len > 200 {
            match level {
                1 => match language {
                    "kazakh" => "Бұл адам көп жазды",
                    "russian" => "Этот человек много написал",
                    _ => "This person wrote a lot"
                },
                2 => match language {
                    "kazakh" => "Бұл адам эссе жазды",
                    "russian" => "Этот товарищ написал эссе",
                    _ => "This person wrote an essay"
                },
                3 => match language {
                    "kazakh" => "Бұл адам роман жазып жатыр",
                    "russian" => "Этот товарищ написал целую лекцию",
                    _ => "This person wrote a damn essay"
                },
                4 => match language {
                    "kazakh" => "Бұл мүрдардың романы жазып жатыр",
                    "russian" => "Этот болтун написал целый роман",
                    _ => "This chatterbox wrote a fucking novel"
                },
                5 => match language {
                    "kazakh" => "Бұл ақымақ романды жазып жатыр",
                    "russian" => "Этот долбоёб написал целый роман",
                    _ => "This moron wrote a fucking novel"
                },
                _ => unreachable!()
            }
        } else if message_len < 5 {
            match level {
                1 => match language {
                    "kazakh" => "Бұл адам аз сөйледі",
                    "russian" => "Этот человек мало сказал",
                    _ => "This person said very little"
                },
                2 => match language {
                    "kazakh" => "Бұл адам сөз таба алмады",
                    "russian" => "Этот человек слов не нашёл",
                    _ => "This person couldn't find words"
                },
                3 => match language {
                    "kazakh" => "Бұл адам сөз табысты жоқ",
                    "russian" => "Этот гений слов не может связать",
                    _ => "This genius can barely form a sentence"
                },
                4 => match language {
                    "kazakh" => "Бұл ақылсыз сөйлей алмайды",
                    "russian" => "Этот тупой даже слова связать не может",
                    _ => "This dummy can't even string words together"
                },
                5 => match language {
                    "kazakh" => "Бұл ақылсыз ештеңе айта алмайды",
                    "russian" => "Этот тупой даже слова связать не может",
                    _ => "This idiot can barely string words together"
                },
                _ => unreachable!()
            }
        } else if is_caps {
            match level {
                1 => match language {
                    "kazakh" => "Бұл адам дауысты жазып жатыр",
                    "russian" => "Этот человек пишет громко",
                    _ => "This person is writing loudly"
                },
                2 => match language {
                    "kazakh" => "Бұл адам дауыстап жазып жатыр",
                    "russian" => "Этот человек кричит буквами",
                    _ => "This person is shouting in text"
                },
                3 => match language {
                    "kazakh" => "Бұл адам дауыстап жазып жатыр",
                    "russian" => "Этот крикун орёт заглавными",
                    _ => "This person is yelling like a maniac"
                },
                4 => match language {
                    "kazakh" => "Бұл дауыстап жазып жатыр, ақылын жоғалтты",
                    "russian" => "Этот псих орёт заглавными как ненормальный",
                    _ => "This psycho is screaming in caps like crazy"
                },
                5 => match language {
                    "kazakh" => "Бұл дауыстап жазып жатыр, ақылын жоғалтты",
                    "russian" => "Этот дебил орёт заглавными как психованный",
                    _ => "This fucking lunatic is screaming in caps"
                },
                _ => unreachable!()
            }
        } else if has_emojis {
            match level {
                1 => match language {
                    "kazakh" => "Бұл адам эмодзи жақсы көреді",
                    "russian" => "Этот человек любит эмодзи",
                    _ => "This person likes emojis"
                },
                2 => match language {
                    "kazakh" => "Бұл эмодзиге ғашық адам",
                    "russian" => "Этот любитель эмодзи",
                    _ => "This emoji lover"
                },
                3 => match language {
                    "kazakh" => "Бұл эмодзиге ғашық адам",
                    "russian" => "Этот любитель эмодзи",
                    _ => "This emoji enthusiast"
                },
                4 => match language {
                    "kazakh" => "Бұл эмодзиге мас болған адам",
                    "russian" => "Этот помешанный на эмодзи",
                    _ => "This emoji-obsessed person"
                },
                5 => match language {
                    "kazakh" => "Бұл эмодзиге мас болған балапан",
                    "russian" => "Этот долбоёб обожает эмодзи как малолетка",
                    _ => "This emoji-obsessed manchild"
                },
                _ => unreachable!()
            }
        } else {
            match level {
                1 => match language {
                    "kazakh" => "Бұл адам айтты",
                    "russian" => "Этот человек сказал",
                    _ => "This person said"
                },
                2 => match language {
                    "kazakh" => "Бұл адам айтты",
                    "russian" => "Этот товарищ сказал",
                    _ => "This individual said"
                },
                3 => match language {
                    "kazakh" => "Бұл адам айтты",
                    "russian" => "Этот персонаж сказал",
                    _ => "This character said"
                },
                4 => match language {
                    "kazakh" => "Бұл жігіт айтты",
                    "russian" => "Этот товарищ заявил",
                    _ => "This guy declared"
                },
                5 => match language {
                    "kazakh" => "Бұл дурак айтты",
                    "russian" => "Этот дебил сказал",
                    _ => "This dumbass said"
                },
                _ => unreachable!()
            }
        };

        context.to_string()
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