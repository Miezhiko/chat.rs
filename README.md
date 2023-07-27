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

# Current state

```bash
running 11 tests
test g4f::forefront::ff_tests::ff_test ... ok
test gpt4free::theb::theb_tests::theb_test ... ok
test gpt4free::aicolors::colour_tests::colour_test ... ok
test opengpt::chatbase::chatbase_tests::chatbase_test ... ok
test g4f::aitianhu::aitianhu_tests::aitianhu_test ... ok
test g4f::deepai::deep_tests::deep_test ... ok
test g4f::getgpt::gg_tests::gg_test ... ok
test g4f::chatgpt_ai::chatgpt_ai_tests::chatgpt_ai_test ... ok
test gpt4free::gptworldAi::world_tests::world_test ... ok
test chimera::chimera_tests::chimera_test ... ok
test poe::poe_tests::poe_test has been running for over 60 seconds
test poe::poe_tests::poe_test ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 100.40s
```
