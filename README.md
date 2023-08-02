# OpenAI How-To Helper

This Rust program provides a simple CLI to interact with OpenAI's GPT* models using the OpenAI API.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [OpenAI API Key](https://openai.com/blog/openai-api)

## Usage

First, build the binary:

```bash
cargo build --release
```

Then you can run the program with:

```bash
export OPENAI_API_KEY=your-api-key
./target/release/howto -m "gpt-3.5-turbo" "Say this is a test!" -t 0.7
```

### Arguments

```bash
-m or --model (optional): The model to use. Defaults to "gpt-3.5-turbo".
-t or --temperature (optional): The temperature for the model to use. Defaults to 0.7.
-x or --max-tokens (optional): The maximum number of tokens to use for the interaction. Defaults to 2048.
message (required): The message content to send to the model.
```

As `--model` and `--temperature` are optional parameters you can just invoke it as:

```bash
./target/release/howto Say this is a test!
```

## Contributing

Contributions are welcome. Please open an issue to discuss your ideas before making large changes.

## License

This project is licensed under the MIT License.
