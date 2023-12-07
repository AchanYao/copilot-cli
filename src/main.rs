mod request_body;

use std::collections::LinkedList;
use clap::{App, Arg};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use dirs;
use sys_info;
use crate::request_body::{Message, OpenAiRequestBody};

#[derive(Serialize, Deserialize)]
struct Config {
    openai_token: String,
    base_url: String,
    max_token: Number,
    model: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Copilot CLI")
        .version("0.1.0")
        .about("Interacts with OpenAI's API to provide shell command suggestions.")
        .arg(
            Arg::new("query")
                .help("The query you want to ask the AI for shell command suggestions.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let config = load_or_create_config()?;
    let system_info = get_system_info()?;
    let response = ask_openai(config, query, &system_info)?;

    println!("{}", response);
    Ok(())
}

fn load_or_create_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".copilot_cli_config.json");

    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;

    if config.openai_token.is_empty() {
        return Err("OpenAI token is missing in the config file. Please add your OpenAI token to the '.copilot_cli_config.json' file in your home directory.".into());
    }

    Ok(config)
}

fn create_default_config(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = Config {
        openai_token: String::new(),
        base_url: String::from("https://api.openai.com/v1"),
        max_token: Number::from(1000),
        model: "gpt-3.5-turbo".to_string(),
    };
    let config_str = serde_json::to_string_pretty(&default_config)?;
    fs::File::create(config_path)?.write_all(config_str.as_bytes())?;

    println!("Created default config file at: {:?}", config_path);
    println!("Please add your OpenAI token to the '.copilot_cli_config.json' file in your home directory and rerun the program.");
    std::process::exit(1);
}

fn get_system_info() -> Result<String, Box<dyn std::error::Error>> {
    let os_type = sys_info::os_type()?;
    let os_release = sys_info::os_release()?;
    let terminal = std::env::var("TERM").unwrap_or_else(|_| "unknown".into());

    Ok(format!("Operating System [{} {}], Terminal Environment [{}]", os_type, os_release, terminal))
}

fn ask_openai(config: Config, query: &str, system_info: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut list = LinkedList::new();

    let system_message = Message {
        role: "system".to_string(),
        content: format!("Context: {}. Goal: To assist users in accomplishing tasks through command line instructions and provide explanations.
Please format the response with a clear separation between the command and the explanation, using the following structure:
Command:
[The command line instruction]

Explanation:
[A step-by-step explanation of what the command does and how it achieves the user's goal.]
Ensure there is a blank line between the 'Command:' and 'Explanation:' sections.
", system_info)
    };
    list.push_back(system_message);

    let user_message = Message {
        role: "user".to_string(),
        content: query.to_string(),
    };
    list.push_back(user_message);

    let body = OpenAiRequestBody {
        model: config.model,
        messages: list,
    };
    let response = client.post(config.base_url + "/chat/completions")
        .bearer_auth(config.openai_token)
        .json(&json!(body))
        .send()?
        .json::<serde_json::Value>()?;

    let command = response["choices"][0]["message"]["content"].as_str().ok_or("Failed to parse the response from OpenAI")?;
    Ok(command.trim().to_string())
}