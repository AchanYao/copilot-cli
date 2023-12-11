# .copilot_cli_config.json

## openai

参照[OpenAI API](https://platform.openai.com/docs/api-reference/chat/create)。

`system_prompt`字段没有特殊要求，留空即可。

```json
{
  "openai_token": "xxxxx",
  "base_url": "https://api.openai.com",
  "request_path": "/v1/chat/completions",
  "max_tokens": 1000,
  "model": "gpt-3.5-turbo",
  "system_prompt": ""
}
```

## 通义千问

请求部分参照[官方文档](https://help.aliyun.com/zh/dashscope/developer-reference/api-details/?spm=a2c4g.11186623.0.0.682416e9QLGu89)Http调用接口部分

`system_prompt`字段没有特殊要求，留空即可。

`ai_type`字段为`DashScope`。（必须）

```json
{
  "openai_token": "sk-xxxx",
  "base_url": "https://dashscope.aliyuncs.com",
  "request_path": "/api/v1/services/aigc/text-generation/generation",
  "max_tokens": 1000,
  "model": "qwen-max",
  "system_prompt": "",
  "ai_type": "DashScope"
}
```