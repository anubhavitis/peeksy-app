use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json;
use tauri::Manager;

use crate::configs::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
    pub user_id: String,
    pub user_email: String,
    pub stored_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthValidation {
    pub is_valid: bool,
    pub is_expired: bool,
    pub is_near_expiry: bool,
    pub expires_in_seconds: Option<i64>,
    pub reason: Option<String>,
}

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

pub struct Store {}

impl Store {
    pub fn fetch(app: tauri::AppHandle) -> Result<Self, String> {
        Ok(Store {})
    }

    /// Fetch auth session data from storage
    pub fn fetch_auth(app: tauri::AppHandle) -> Result<AuthSession, String> {
        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        let auth_file_path = app_data_path.join("auth_session.json");

        if !auth_file_path.exists() {
            return Err("No auth session found".to_string());
        }

        let auth_file =
            File::open(&auth_file_path).map_err(|e| format!("Failed to open auth file: {}", e))?;

        let auth_session: AuthSession = serde_json::from_reader(auth_file)
            .map_err(|e| format!("Failed to parse auth session: {}", e))?;

        Ok(auth_session)
    }

    /// Save auth session data to storage
    pub fn save_auth(app: tauri::AppHandle, session: &AuthSession) -> Result<(), String> {
        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;

        let auth_file_path = app_data_path.join("auth_session.json");
        let auth_file = File::create(&auth_file_path)
            .map_err(|e| format!("Failed to create auth file: {}", e))?;

        serde_json::to_writer_pretty(auth_file, session)
            .map_err(|e| format!("Failed to write auth session: {}", e))?;

        Ok(())
    }

    /// Validate if the auth session is still valid
    pub fn validate_auth(session: &AuthSession) -> AuthValidation {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check if session is too old (7 days)
        let max_age = 7 * 24 * 60 * 60; // 7 days in seconds
        let session_age = current_time - session.stored_at;

        if session_age > max_age {
            return AuthValidation {
                is_valid: false,
                is_expired: true,
                is_near_expiry: false,
                expires_in_seconds: None,
                reason: Some("Session is older than 7 days".to_string()),
            };
        }

        // Check token expiry if expires_at is provided
        if let Some(expires_at_str) = &session.expires_at {
            match expires_at_str.parse::<u64>() {
                Ok(expires_at) => {
                    let expires_in = expires_at as i64 - current_time as i64;

                    if expires_in <= 0 {
                        return AuthValidation {
                            is_valid: false,
                            is_expired: true,
                            is_near_expiry: false,
                            expires_in_seconds: Some(expires_in),
                            reason: Some("Access token has expired".to_string()),
                        };
                    }

                    // Check if expiring within 5 minutes
                    let near_expiry = expires_in < 300; // 5 minutes

                    return AuthValidation {
                        is_valid: true,
                        is_expired: false,
                        is_near_expiry: near_expiry,
                        expires_in_seconds: Some(expires_in),
                        reason: if near_expiry {
                            Some("Token expires soon".to_string())
                        } else {
                            None
                        },
                    };
                }
                Err(_) => {
                    // If we can't parse expires_at, assume it's valid but warn
                    return AuthValidation {
                        is_valid: true,
                        is_expired: false,
                        is_near_expiry: false,
                        expires_in_seconds: None,
                        reason: Some("Could not parse expiry time".to_string()),
                    };
                }
            }
        }

        // If no expires_at, session is valid but we don't know expiry
        AuthValidation {
            is_valid: true,
            is_expired: false,
            is_near_expiry: false,
            expires_in_seconds: None,
            reason: None,
        }
    }

    /// Check if user is authenticated and session is valid
    pub fn is_authenticated(app: tauri::AppHandle) -> Result<AuthValidation, String> {
        match Self::fetch_auth(app) {
            Ok(session) => Ok(Self::validate_auth(&session)),
            Err(e) => Ok(AuthValidation {
                is_valid: false,
                is_expired: false,
                is_near_expiry: false,
                expires_in_seconds: None,
                reason: Some(e),
            }),
        }
    }

    /// Clear auth session from storage
    pub fn clear_auth(app: tauri::AppHandle) -> Result<(), String> {
        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        let auth_file_path = app_data_path.join("auth_session.json");

        if auth_file_path.exists() {
            std::fs::remove_file(&auth_file_path)
                .map_err(|e| format!("Failed to remove auth file: {}", e))?;
        }

        Ok(())
    }

    /// Fetch Peeksy configuration from storage
    pub fn fetch_peeksy_config(app: tauri::AppHandle) -> Result<PeeksyConfig, String> {
        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        let config_file_path = app_data_path.join("peeksy_config.json");

        if !config_file_path.exists() {
            // Return default config if file doesn't exist
            let default_config = PeeksyConfig::default();
            // Save the default config for future use
            Self::save_peeksy_config(app, &default_config)?;
            return Ok(default_config);
        }

        let config_file = File::open(&config_file_path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;

        let config: PeeksyConfig = serde_json::from_reader(config_file)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(config)
    }

    /// Save Peeksy configuration to storage
    pub fn save_peeksy_config(app: tauri::AppHandle, config: &PeeksyConfig) -> Result<(), String> {
        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;

        let config_file_path = app_data_path.join("peeksy_config.json");
        let config_file = File::create(&config_file_path)
            .map_err(|e| format!("Failed to create config file: {}", e))?;

        // Update timestamp before saving
        let mut updated_config = config.clone();
        updated_config.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        serde_json::to_writer_pretty(config_file, &updated_config)
            .map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    /// Update specific configuration field
    pub fn update_config_field(
        app: tauri::AppHandle,
        field: &str,
        value: &str,
    ) -> Result<PeeksyConfig, String> {
        let mut config = Self::fetch_peeksy_config(app.clone())?;

        match field {
            "openai_api_key" => config.openai_api_key = value.to_string(),
            "peeksy_prompt" => config.peeksy_prompt = value.to_string(),
            "openai_model" => config.openai_model = value.to_string(),
            _ => return Err(format!("Unknown config field: {}", field)),
        }

        Self::save_peeksy_config(app, &config)?;
        Ok(config)
    }

    /// Validate configuration completeness
    pub fn validate_peeksy_config(config: &PeeksyConfig) -> Vec<String> {
        let mut missing_fields = Vec::new();

        if config.openai_api_key.trim().is_empty() {
            missing_fields.push("OpenAI API Key".to_string());
        }

        if config.peeksy_prompt.trim().is_empty() {
            missing_fields.push("Peeksy Prompt".to_string());
        }

        if config.openai_model.trim().is_empty() {
            missing_fields.push("OpenAI Model".to_string());
        }

        missing_fields
    }

    /// Check if configuration is complete and valid
    pub fn is_config_complete(app: tauri::AppHandle) -> Result<bool, String> {
        let config = Self::fetch_peeksy_config(app)?;
        let missing_fields = Self::validate_peeksy_config(&config);
        Ok(missing_fields.is_empty())
    }

    /// Reset configuration to defaults
    pub fn reset_peeksy_config(app: tauri::AppHandle) -> Result<PeeksyConfig, String> {
        let default_config = PeeksyConfig::default();
        Self::save_peeksy_config(app, &default_config)?;
        Ok(default_config)
    }

    /// Get configuration status for debugging
    pub fn get_config_status(app: tauri::AppHandle) -> Result<(PeeksyConfig, Vec<String>), String> {
        let config = Self::fetch_peeksy_config(app)?;
        let missing_fields = Self::validate_peeksy_config(&config);
        Ok((config, missing_fields))
    }
}
