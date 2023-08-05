use aionic::openai::{Chat, OpenAIClient};
use clap::{arg, value_parser, ArgAction, Command};
use std::error::Error;

fn primer() -> String {
    "You're a very smart life assisstant versatile in all possible things. 
     Your task is to give me a very concise on point answer to the questions I'm about to ask you"
        .to_string()
}

fn clap_cli() -> clap::ArgMatches {
    Command::new("howto-openai")
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
                .default_value("0.5"),
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
                .default_value("false"),
        )
        .arg(
            arg!([MESSAGE])
                .action(ArgAction::Append)
                .required(true),
        )
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let matches = clap_cli();
    let model = matches.get_one::<String>("MODEL").unwrap();
    let temperature = matches.get_one::<f64>("TEMPERATURE").unwrap().to_owned();
    let max_tokens = matches.get_one::<u64>("MAX_TOKENS").unwrap().to_owned();
    let is_stream = !matches.get_one::<bool>("STREAM").unwrap().to_owned();
    let mut message: String = matches
        .get_many::<String>("MESSAGE")
        .unwrap_or_default()
        .map(std::string::String::as_str)
        .collect::<Vec<_>>()
        .join(" ");

    if message.is_empty() {
        println!("No message given");
        return Ok(());
    }
    if !message.starts_with("how") {
        message = format!("how to {}", message);
    }

    let temperature = if (&0.0..=&1.0).contains(&&temperature) {
        temperature
    } else {
        1.0
    };

    OpenAIClient::<Chat>::new()
        .set_model(model.clone())
        .set_temperature(temperature)
        .set_max_tokens(max_tokens)
        .set_stream_responses(is_stream)
        .set_primer(primer())
        .ask(message.clone())
        .await?;

    Ok(())
}
