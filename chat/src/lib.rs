#[macro_use] extern crate anyhow;

mod personality;
mod constants;

pub mod gpt4free;
pub mod opengpt;
pub mod g4f;
pub mod chimera;

pub async fn generate(msg: &str) -> anyhow::Result<String> {
  if let Ok(gpt4free_result)        = chimera::generate( msg, false, "Amadeus" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = opengpt::chatbase::generate( msg ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::chatgpt_ai::generate( msg, true, "Amadeus" ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::gptworldAi::generate( msg, true, "Amadeus" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::getgpt::generate( msg, true, "Amadeus" ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::theb::generate( msg ) {
    Ok(gpt4free_result)
  } else { Err(anyhow!("Failed to generate chat response")) }
}

pub async fn chat(msg: &str, bot_name: &str) -> anyhow::Result<String> {
  let fmode =
    ! (msg.contains("please")
    || msg.contains("пожалуйста")
    || msg.contains("Please")
    || msg.contains("Пожалуйста")
    || msg.contains("PLEASE"));

  if let Ok(gpt4free_result)        = chimera::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::chatgpt_ai::generate( msg, true, bot_name ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::gptworldAi::generate( msg, fmode, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = opengpt::chatbase::generate( msg ) {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = g4f::getgpt::generate( msg, true, bot_name ).await {
    Ok(gpt4free_result)
  } else if let Ok(gpt4free_result) = gpt4free::theb::generate( msg ) {
    Ok(gpt4free_result)
  } else { Err(anyhow!("Failed to generate chat response")) }
}
