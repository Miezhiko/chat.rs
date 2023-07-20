# chat.rs

[![mawa](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml)

```rust
match command {
  COLORS => {
    if let Ok(gpt4free_result) =
      chat::gpt4free::aicolors::generate( payload )
    {
      return Some((k_key, gpt4free_result));
    }
  }, GENERATE => {
    if let Ok(gpt4free_result) =
      chat::generate( payload )
    {
      return Some((k_key, gpt4free_result));
    }
  }, CHAT => {
    if let Ok(gpt4free_result) =
      chat::chat( payload, "bot name" )
    {
      return Some((k_key, gpt4free_result));
    }
  }, _ => return None;
};
```
