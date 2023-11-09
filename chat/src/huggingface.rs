use crate::types::Generator;

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use async_trait::async_trait;

pub struct HuggingFaceGenerator;

#[async_trait]
impl Generator for HuggingFaceGenerator {
  fn name<'a>( &self ) -> &'a str {
    "HuggingFace"
  }
  fn enabled_for_multigen( &self ) -> bool {
    false
  }
  async fn call( &self
               , prompt: &str
               , _fmode: bool
               , _personality: &str )
    -> anyhow::Result<String> {
    match catch_unwind(|| {
      let c = Context::new();
      c.set("prompt", prompt);
      c.run(python! {
        from huggingface_hub import InferenceClient
        import os
      
        if os.path.isfile("/etc/chat.rs/hugging.txt"):
            file_path = "/etc/chat.rs/hugging.txt"
        else:
            file_path = "hugging.txt"
        with open(file_path, "r") as file:
          htoken = file.readline().strip()
        client = InferenceClient(token=htoken)
        reslt = False
        try:
          rspns = client.conversational(prompt)

          if not rspns:
            result = "huggingface: Sorry, I can't generate a response right now."
          else:
            reslt = True
            result = rspns["generated_text"]
        except OSError as err:
          result = ("OS Error! {0}".format(err))
        except RuntimeError as err:
          result = ("Runtime Error! {0}".format(err))
      }); ( c.get::<bool>("reslt")
          , c.get::<String>("result") )
    }) {
      Ok((r,m)) => {
        if r {
          Ok(m)
        } else {
          bail!("No tokens generated: {:?}", m)
        }
      }, Err(_) => { bail!("Failed to to use huggingface now!") }
    }
  }
}

#[cfg(test)]
mod huggingface_tests {
  use super::*;
  #[tokio::test]
  async fn huggingface_tests() {
    let gen = HuggingFaceGenerator;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
