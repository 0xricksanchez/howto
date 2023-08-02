use clap::{arg, value_parser, ArgAction, Command};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f64>,
    stream: Option<bool>,
    max_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Response {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<Choice>>,
    usage: Option<Usage>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u64,
}

fn primer() -> Message {
    Message {
        role: "system".to_string(),
        content: "You're a very smart life assisstant versatile in all possible things. 
                  Your task is to give me a very concise on point answer to the questions I'm about to ask you".to_string(),
    }
}

async fn prompt_openai(client: &Client, prompt: &Prompt) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;

    let url = "https://api.openai.com/v1/chat/completions";

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(prompt)
        .send()
        .await?
        .json::<Response>()
        .await?;

    for choice in res.choices.unwrap() {
        println!("{}: {}", choice.message.role, choice.message.content);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("howto-openai")
        .version("0.1.0")
        .author("0x434b <admin@0x434b.dev>")
        .about("Let openAI help you will simple howto tasks")
        .arg(
            arg!([MODEL])
                .short('m')
                .long("model")
                .help("The openAI model to use")
                .default_value("gpt-3.5-turbo"),
        )
        .arg(
            arg!([TEMPERATURE])
                .short('t')
                .long("temperature")
                .help("The temperature to use for the model. Higher values mean more random results. A value between 0.0 and 1.0!")
                .value_parser(value_parser!(f64))
                .default_value("0.7"),
        )
        .arg(
            arg!([MAX_TOKENS])
                .short('x')
                .long("max-tokens")
                .help("The maximum number of tokens to generate. Between 1 and 2048")
                .value_parser(value_parser!(u64).range(1..=2048))
                .default_value("2048"),
        )
        .arg(
            arg!([MESSAGE])
                .action(ArgAction::Append)
                .required(true),
        )
        .get_matches();
    let model = matches.get_one::<String>("MODEL").unwrap();
    let temperature = matches.get_one::<f64>("TEMPERATURE").unwrap().to_owned();
    let max_tokens = matches.get_one::<u64>("MAX_TOKENS").unwrap().to_owned();
    let mut message: String = matches
        .get_many::<String>("MESSAGE")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    if message.is_empty() {
        println!("No message given");
        return Ok(());
    }
    if !message.starts_with("how") {
        message = format!("how to {}", message);
    }

    let temperature = if !(&0.0..=&1.0).contains(&&temperature) {
        1.0
    } else {
        temperature
    };

    let client = Client::new();

    let prompt = Prompt {
        model: model.to_owned(),
        messages: vec![
            primer(),
            Message {
                role: "user".to_string(),
                content: message,
            },
        ],
        temperature: Some(temperature),
        max_tokens: Some(max_tokens),
        stream: Some(false),
    };

    prompt_openai(&client, &prompt).await?;
    Ok(())
}
