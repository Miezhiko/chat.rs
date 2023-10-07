# chat.rs

[![mawa](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml)

# Requirements:

- [poe-api](https://github.com/ading2210/poe-api) (deprecated, dropped)
- [gpt4free](https://github.com/xtekky/gpt4free)
- [OpenAI](https://github.com/openai/openai-python)
- [OpenGPT](https://github.com/uesleibros/OpenGPT) (deprecated, dropped)
- [gpt4free fork](https://github.com/Masha/gpt4free) (deprecated, dropped)
- [ChimeraGPT](https://discord.gg/chimeragpt) access token as `chimera.txt` file (dead)

# Silly usage example

```rust
if let Ok(gpt4free_result) =
  chat::chat( payload, "bot name" ) {
    return Some(gpt4free_result);
  }
};
```

# Current state

```rust
static GENERATORS: Lazy<Vec<Arc<dyn Generator + Send + Sync>>> =
  Lazy::new(|| {
    vec![ Arc::new( g4f::chatbase::ChatBaseGenerator )
        , Arc::new( g4f::wewordle::WewordleGenerator )
        , Arc::new( g4f::chatgptduo::ChatgptDuoGenerator )
        , Arc::new( g4f::ylokh::YlokhGenerator )
        , Arc::new( g4f::gptgo::GptGoGenerator )
        , Arc::new( g4f::gptgod::GptGodGenerator )
        , Arc::new( g4f::yqcloud::YqcloudGenerator )
        , Arc::new( g4f::myshell::MyshellGenerator )
        , Arc::new( g4f::aibn::AibnGenerator )
        , Arc::new( g4f::phind::PhindGenerator)
        , Arc::new( g4f::deepai::DeepAiGenerator )
        , Arc::new( g4f::freegpt::FreeGptGenerator )
        , Arc::new( g4f::chatgptai::ChatgptAiGenerator )
        , Arc::new( g4f::aitianhuspace::AItianhuSpaceGenerator )
        , Arc::new( g4f::gptforlove::GptForLoveGenerator )
        ]
  });

pub async fn generate(msg: &str, bot_name: &str, fancy: bool) -> anyhow::Result<String> {
  for gen in &*GENERATORS {
    if let Ok(result) = gen.call(msg, fancy, bot_name).await {
      return Ok(result);
    }
  }
  Err( anyhow::anyhow!("All generators failed") )
}

pub async fn generate_all<'a>(msg: &str, bot_name: &str, fancy: bool)
                                -> Vec<(&'a str, anyhow::Result<String>)> {
  let genz = (&*GENERATORS).into_iter().map(
    |gen| async move { ( gen.name()
                       , gen.call(msg, fancy, bot_name).await )
                     }
  );
  future::join_all(genz).await
}
```
