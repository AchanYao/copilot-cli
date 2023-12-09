mod request_body;
mod runtime_config;

use std::collections::LinkedList;
use clap::{App, Arg};
use reqwest;
use serde_json::json;
use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;
use dirs;
use sys_info;
use crate::request_body::{Message, OpenAiRequestBody};
use crate::runtime_config::{GLOBAL_CONFIG, RuntimeConfig};

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

    // load config
    load_or_create_config()?;

    let response = ask_openai(query)?;

    println!("{}", response);
    Ok(())
}

fn load_or_create_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".copilot_cli_config.json");

    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let config_str = fs::read_to_string(config_path)?;
    let mut config = GLOBAL_CONFIG.write().unwrap();
    config.copy_from_json(config_str);

    if config.openai_token().is_empty() {
        return Err("OpenAI token is missing in the config file. Please add your OpenAI token to the '.copilot_cli_config.json' file in your home directory.".into());
    }

    Ok(())
}

fn create_default_config(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = GLOBAL_CONFIG.read().unwrap();
    let config_str = serde_json::to_string_pretty(&*config)?;
    fs::File::create(config_path)?.write_all(config_str.as_bytes())?;

    println!("Created default config file at: {:?}", config_path);
    println!("Please add your OpenAI token to the '.copilot_cli_config.json' file in your home directory and rerun the program.");
    std::process::exit(1);
}

fn get_system_info() -> Result<String, Box<dyn std::error::Error>> {
    let os_type = sys_info::os_type()?;
    let os_release = sys_info::os_release()?;

    let config = GLOBAL_CONFIG.read().unwrap();
    let terminal = get_terminal_name()
        .unwrap_or(config.default_shell());

    Ok(format!("Operating System [{} {}], Terminal Environment [{}]", os_type, os_release, terminal))
}

fn get_terminal_name() -> Option<String> {
    let shell = if let Ok(shell) = env::var("SHELL") {
        if shell.contains("/bash") {
            Some("bash")
        } else if shell.contains("/zsh") {
            Some("zsh")
        } else {
            None
        }
    } else if env::var("PSModulePath").is_ok() {
        Some("PowerShell")
    } else if let Ok(comspec) = env::var("COMSPEC") {
        if comspec.to_lowercase().ends_with("cmd.exe") {
            Some("cmd")
        } else {
            None
        }
    } else {
        None
    };

    return shell.map(|s| s.to_string());
}

fn ask_openai(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = &GLOBAL_CONFIG.read().unwrap();

    let system_info = get_system_info()?;

    let client = reqwest::blocking::Client::new();
    let mut list = LinkedList::new();

    let system_info_explanation = format!("Operating System is reliable and should be used as the default. Terminal Environment is determined by the program and may not match user expectations. If the user specifies a different terminal within their request, prioritize the user's choice. For OS-specific queries, ignore user preferences for an OS different from [OS], except when the question pertains to different distributions of the same OS, in which case use discretion. Goal: To provide users with executable command line instructions for the current environment and terminal, and offer explanations that are easily readable, using line breaks or tabs to enhance readability. If the user's language is clear, respond in kind; otherwise, default to English.");

    let system_message = Message {
        role: "system".to_string(),
        content: std::fmt::format(format_args!("{}\n{}\n{}", system_info, system_info_explanation, config.system_prompt()))
    };
    list.push_back(system_message);

    let user_message = Message {
        role: "user".to_string(),
        content: query.to_string(),
    };
    list.push_back(user_message);

    let body = OpenAiRequestBody {
        model: config.model(),
        messages: list,
        max_tokens: config.max_tokens(),
    };
    let response = client.post(config.base_url() + "/chat/completions")
        .bearer_auth(config.openai_token())
        .json(&json!(body))
        .send()?
        .json::<serde_json::Value>()?;

    let command = response["choices"][0]["message"]["content"].as_str().ok_or("Failed to parse the response from OpenAI")?;
    Ok(command.trim().to_string())
}