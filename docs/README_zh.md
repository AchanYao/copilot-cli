# Copilot Cli

> 基于OpenAI实现的命令行助手

一个帮你生成命令的命令行工具，基于OpenAI的GPT模型。

# Usage

```bash
cargo run "list all rs files in current directory"
```

or

```bash
# 如果你使用的是二进制文件，你可以直接使用copilot-cli
# 你可能需要将它加入到你的PATH中
copilot-cli "list all rs files in current directory"
```

当首次运行的时候，它会在用户目录下创建一个`.copilot_cli_config.json`配置文件，你需要在里面填入你的OpenAI API key。

```json
{
  "openai_token": "your openai token"
}
```

你可以在[这里](https://platform.openai.com/api-keys)申请一个OpenAI API key。

中国用户可以使用[通义千问](https://help.aliyun.com/zh/dashscope/developer-reference/activate-dashscope-and-create-an-api-key)。

你可以修改配置文件来改变默认设置，更多信息可以看[这里](../src/runtime_config.rs)。

# Enjoying

![demo](./images/demo.gif)

# Why This Project

> 为什么要做这个项目

项目灵感来源于微软的[Copilot for CLI](https://githubnext.com/projects/copilot-cli/)，
为什么我不直接用微软的，因为我内测资格至今没有申请到