# chat.rs

[![mawa](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml)

# Requirements:

- [poe-api](https://github.com/ading2210/poe-api) (deprecated, dropped)
- [OpenGPT](https://github.com/uesleibros/OpenGPT)
- [gpt4free](https://github.com/xtekky/gpt4free)
- [OpenAI](https://github.com/openai/openai-python)
- [OpenGPT](https://github.com/uesleibros/OpenGPT) (deprecated, dropped)
- [gpt4free fork](https://github.com/Masha/gpt4free) (deprecated, dropped)
- [ChimeraGPT](https://discord.gg/chimeragpt) access token as `chimera.txt` file

# Silly usage example

```rust
if let Ok(gpt4free_result) =
  chat::chat( payload, "bot name" ) {
    return Some(gpt4free_result);
  }
};
```

# Current state

```bash
test g4f::opchatgpts::opchatgpts_tests::opchatgpts_test ... ok
test gpt4free::theb::theb_tests::theb_test ... ok
test g4f::deepai::deepai_tests::deepai_test ... ok
test opengpt::chatbase::chatbase_tests::chatbase_test ... ok
test g4f::ails::ails_tests::ails_test ... ok
test g4f::chatgpt_ai::chatgpt_ai_tests::chatgpt_ai_test ... ok
test g4f::yqcloud::yqcloud_tests::yqcloud_test ... ok
test chimera::chimera_tests::chimera_test ... ok
test gpt4free::gptworldAi::world_tests::world_test ... ok
test g4f::getgpt::gg_tests::gg_test ... ok
test phind::phind_tests::phind_test ... ok
```
