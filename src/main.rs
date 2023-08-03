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

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

impl Prompt {
    fn new(model: String, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            temperature: None,
            stream: None,
            max_tokens: None,
        }
    }

    fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    fn _get_client(&mut self) -> Client {
        Client::new()
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let api_key = env::var("OPENAI_API_KEY")?;
        if self.stream.unwrap_or(false) {
            self._stream_openai(&api_key).await?;
        } else {
            self._prompt_openai(&api_key).await?;
        }

        Ok(())
    }

    async fn _stream_openai(&mut self, api_key: &str) -> Result<(), Box<dyn Error>> {
        let client = self._get_client();
        let mut res = client
            .post(OPENAI_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&self)
            .send()
            .await?;

        while let Some(chunk) = res.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            let lines: Vec<&str> = chunk_str.split('\n').collect();
            for line in lines {
                if let Some(chunk) = line.strip_prefix("data: ") {
                    let serde_chunk: Result<StreamedReponse, _> = serde_json::from_str(chunk);
                    match serde_chunk {
                        Ok(chunk) => {
                            for choice in chunk.choices {
                                if let Some(content) = choice.delta.content {
                                    print!(
                                        "{}",
                                        content.trim().strip_suffix('\n').unwrap_or(&content)
                                    );
                                }
                            }
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn _prompt_openai(&mut self, api_key: &str) -> Result<(), Box<dyn Error>> {
        let client = self._get_client();
        let res = client
            .post(OPENAI_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&self)
            .send()
            .await?
            .json::<Response>()
            .await?;

        for choice in res.choices.unwrap() {
            println!("{}: {}", choice.message.role, choice.message.content);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamedReponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<StreamedChoices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamedChoices {
    index: u64,
    delta: Delta,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    role: Option<String>,
    content: Option<String>,
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
            arg!([STREAM])
                .short('s')
                .long("stream")
                .help("Disable streaming the output from openAI.")
                .required(false)
                .action(ArgAction::SetFalse)
                .default_value("true")
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
    let is_stream = matches.get_one::<bool>("STREAM").unwrap().to_owned();
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

    Prompt::new(
        model.to_owned(),
        vec![
            primer(),
            Message {
                role: "user".to_string(),
                content: message,
            },
        ],
    )
    .with_temperature(temperature)
    .with_max_tokens(max_tokens)
    .with_stream(is_stream)
    .run()
    .await?;

    Ok(())
}
