use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub openai_prompt_file_path: Option<String>,
    pub openai_model: Option<String>,
}

impl Config {
    pub fn get() -> Self {
        // run command peeksy current-configs on cli
        let output = Command::new("peeksy")
            .arg("current-config")
            .output()
            .unwrap();
        let output_str = String::from_utf8(output.stdout).unwrap();
        let config_string = Self::filter(&output_str);
        let config: Config = serde_json::from_str(&config_string).unwrap();
        config
    }

    fn filter(output: &str) -> String {
        let output_lines = output.lines();
        let mut i = 0;
        let mut j = 0;
        for (index, line) in output_lines.clone().enumerate() {
            if line.starts_with("{") {
                i = index;
            }
            if line.starts_with("}") {
                j = index;
            }
        }

        let config_string = output_lines
            .into_iter()
            .skip(i)
            .take(j - i + 1)
            .collect::<Vec<&str>>()
            .join("\n");
        config_string
    }
}
