# chat.rs

[![mawa](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml)

# Requirements:

- [poe-api](https://github.com/ading2210/poe-api)
- [poe api token](https://discord.gg/N9ejkJ4uaB) as `tokens.txt` file
- [OpenGPT](https://github.com/uesleibros/OpenGPT)
- [gpt4free](https://github.com/xtekky/gpt4free)
- [OpenAI](https://github.com/openai/openai-python)
- [OpenGPT](https://github.com/uesleibros/OpenGPT)
- [gpt4free fork](https://github.com/Masha/gpt4free) (this is optional and will be dropped)
- [ChimeraGPT](https://discord.gg/chimeragpt) access token as `chimera.txt` file

# Silly usage example

```rust
match command {
  COLORS => {
    if let Ok(gpt4free_result) =
      chat::gpt4free::aicolors::generate( payload )
    {
      return Some(gpt4free_result);
    }
  }, GENERATE => {
    if let Ok(gpt4free_result) =
      chat::generate( payload )
    {
      return Some(gpt4free_result);
    }
  }, CHAT => {
    if let Ok(gpt4free_result) =
      chat::chat( payload, "bot name" )
    {
      return Some(gpt4free_result);
    }
  }, _ => return None;
};
```
