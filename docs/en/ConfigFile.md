# .copilot_cli_config.json

## OpenAI

Refer to [OpenAI API](https://platform.openai.com/docs/api-reference/chat/create).

The `system_prompt` field has no special requirements; it can be left empty.

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

## Tongyi Qianwen

For the request part, refer to the Http invocation interface section of the [official documentation](https://help.aliyun.com/zh/dashscope/developer-reference/api-details/?spm=a2c4g.11186623.0.0.682416e9QLGu89).

The `system_prompt` field has no special requirements; it can be left empty.

The `ai_type` field should be `DashScope`. (This is mandatory.)

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