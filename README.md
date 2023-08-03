# OpenAI How-To Helper

This Rust program provides a simple CLI to interact with OpenAI's GPT* models using the OpenAI API.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [OpenAI API Key](https://openai.com/blog/openai-api)

## Setup

First, build the binary:

```bash
cargo build --release
```

Then you can run the program with:

```bash
export OPENAI_API_KEY=your-api-key
cargo build
./target/debug/howto get closer to world peace
Promote open dialogue, empathy, and understanding between different cultures and nations. Encourage diplomacy, conflict resolution, and cooperation on global issues. Foster education, tolerance, and respect for diversity. Support organizations and initiatives dedicated to peacebuilding and non-violence.
```

### Usage

```bash
Usage: howto [OPTIONS] <MESSAGE>...

Arguments:
  <MESSAGE>...  

Options:
  -m, --model <MODEL>              The openAI model to use [default: gpt-3.5-turbo]
  -t, --temperature <TEMPERATURE>  The temperature to use for the model. Higher values mean more random results. A value between 0.0 and 1.0! [default: 0.5]
  -x, --max-tokens <MAX_TOKENS>    The maximum number of tokens to generate. Between 1 and 2048 [default: 2048]
  -s, --stream                     Disable streaming the output from openAI.
  -h, --help                       Print help
  -V, --version                    Print version
```

## Contributing

Contributions are welcome. Please open an issue to discuss your ideas before making large changes.

## License

This project is licensed under the MIT License.
