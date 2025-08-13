use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeeksyConfig {
    pub openai_api_key: String,
    pub peeksy_prompt: String,
    pub openai_model: String,
    pub updated_at: u64,
}

impl Default for PeeksyConfig {
    fn default() -> Self {
        Self {
            openai_api_key: String::new(),
            peeksy_prompt: r#"Analyze the attached image and generate a short, descriptive filename that clearly reflects its subject, context, and content.
Rules:
    1. Use lowercase letters only. Separate words with hyphens. No spaces or underscores.
    2. Keep the filename between 3 to 8 words. Be concise but meaningful.
    3. Apply intelligent context recognition:
        - If it is an album cover, include the album title and band or artist name.
        - If it is artwork, mention the style (e.g., oil-painting, digital-art, 3d-render).
        - If it's a poster, include the movie/show/event name.
    4. Avoid generic terms like "image", "picture", "photo", or "screenshot".
    5. Do not include the file extension (e.g., .jpg or .png) in the output.

Return only the final filename string, with no extra explanation or punctuation."#.to_string(),
            openai_model: "gpt-4o".to_string(),
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}
