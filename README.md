# Copilot Cli

> Copilot for Command Line Interface by OpenAI.

A command line tool that can help you generate commands by OpenAI's GPT model.

# Usage

```bash
cargo run "list all rs files in current directory"
```

or

```bash
# if you have installed it, you can use copilot_cli directly
# maybe you need to add it to your PATH
copilot_cli "list all rs files in current directory"
```

When first run, it will create a `.copilot_cli_config.json` config file in your home directory. You need to fill in your OpenAI API key in it.

```json
{
  "openai_token": "your openai token"
}
```

You can modify the config file to change the default settings. Config file fields:

- `openai_token`: your openai token
- `model`: the model you want to use, default is `gpt-3.5-turbo`
- `system_prompt`: the prompt you want to use
- `max_tokens`: the max tokens you want to generate, default is 1000
- `base_url`: the base url of openai api, default is `https://api.openai.com/v1`
- `default_shell`: the default shell you want to use, if not set, it will judge by your os, if your os is windows, it will use `cmd`, else it will use `bash`

# Enjoying

![demo](./docs/images/demo.gif)