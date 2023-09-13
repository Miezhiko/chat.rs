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

```bash
test chimera::chimera_tests::chimera_test ... ignored
test g4f::chatbase::chatbase_tests::chatbase_test ... ok
test g4f::deepai::deepai_tests::deepai_test ... ok
test g4f::aivvm::aivvm_tests::aivvm_test ... ok
test huggingface::huggingface_tests::huggingface_test ... ok
test g4f::vitalentum::vitalentum_tests::vitalentum_test ... ok
test g4f::codelinkava::codelinkava_tests::codelinkava_test ... ok
test g4f::wewordle::wewordle_tests::wewordle_test ... ok
test g4f::chatgptai::chatgptai_tests::chatgptai_test ... ok
test g4f::yqcloud::yqcloud_tests::yqcloud_test ... ok
test g4f::chatgptlogin::chatgptai_tests::chatgptai_test ... ok
test phind::phind_tests::phind_test ... ok
```

# Nothing

```rust
static GENERATORS: Lazy<Vec<Arc<dyn Generator + Send + Sync>>> =
  Lazy::new(|| {
    vec![ Arc::new( g4f::chatbase::ChatBaseGenerator )
        , Arc::new( g4f::wewordle::WewordleGenerator )
        , Arc::new( g4f::yqcloud::YqcloudGenerator )
        , Arc::new( g4f::chatgptlogin::ChatgptLoginGenerator )
        , Arc::new( phind::PhindGenerator)
        , Arc::new( g4f::aitianhu::AItianhuGenerator )
        , Arc::new( g4f::codelinkava::CodeLinkAvaGenerator )
        , Arc::new( g4f::deepai::DeepAiGenerator )
        , Arc::new( g4f::chatgptai::ChatgptAiGenerator )
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
