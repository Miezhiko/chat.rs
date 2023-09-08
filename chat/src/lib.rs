#[macro_use] extern crate anyhow;

mod personality;
mod constants;

pub mod g4f;
pub mod chimera;
pub mod phind;
pub mod huggingface;

pub async fn generate(msg: &str, bot_name: &str, fancy: bool) -> anyhow::Result<String> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  if let Ok(gpt4free_result)        = chimera::generate( msg, fmode, bot_name, "gpt-3.5-turbo-16k" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = chimera::generate( msg, fmode, bot_name, "llama-2-70b-chat" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::yqcloud::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::wewordle::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = phind::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::aitianhu::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::deepai::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::chatgptlogin::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::chatgptai::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = huggingface::generate( msg ) {
    Ok(gpt4free_result)
  } else { Err(anyhow!("Failed to generate chat response")) }
}

pub async fn generate_all<'a>(msg: &str, bot_name: &str, fancy: bool)
                                -> Vec<(&'a str, anyhow::Result<String>)> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  vec![ ( "llama-2-70b-chat"
        , chimera::generate( msg, fmode, bot_name, "llama-2-70b-chat" ).await )
      , ( "Phind"
        , phind::generate( msg, fmode, bot_name ).await )
      , ( "Yqcloud"
        , g4f::yqcloud::generate( msg, true, bot_name ).await )
      , ( "Wewordle"
        , g4f::wewordle::generate( msg, true, bot_name ).await )
      , ( "AItianhu"
        , g4f::aitianhu::generate( msg, true, bot_name ).await )
      , ( "Deep AI"
        , g4f::deepai::generate( msg, true, bot_name ).await )
      , ( "Chatgpt Login"
        , g4f::chatgptlogin::generate( msg, true, bot_name ).await )
      , ( "ChatgptAI"
        , g4f::chatgptai::generate( msg, true, bot_name ).await )
      , ( "gpt-3.5-turbo-16k"
        , chimera::generate( msg, fmode, bot_name, "gpt-3.5-turbo-16k" ).await )
      ]
}

pub async fn chat(msg: &str, bot_name: &str) -> anyhow::Result<String> {
  generate(msg, bot_name, true).await
}
