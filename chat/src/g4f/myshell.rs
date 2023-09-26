use crate::types::Generator;

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use async_trait::async_trait;

pub struct MyshellGenerator;

#[async_trait]
impl Generator for MyshellGenerator {
  fn name<'a>( &self ) -> &'a str {
    "Myshell"
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
        import sys
        import os
        import g4f
        result = ""
        messages = []
        try:
          messages.append({"role": "user", "content": prompt})
          rspns = g4f.ChatCompletion.create( model="gpt-3.5-turbo", messages=messages
                                           , stream=False, auth="cookies"
                                           , provider=g4f.Provider.Myshell )
          if not rspns:
            result = "Myshell: Sorry, I can't generate a response right now."
            reslt = False
          else:
            reslt = True
            result = rspns
        except OSError as err:
          result = ("OS Error! {0}".format(err))
          reslt = False
        except RuntimeError as err:
          result = ("Runtime Error! {0}".format(err))
          reslt = False
      }); ( c.get::<bool>("reslt")
          , c.get::<String>("result") )
    }) {
      Ok((r,m)) => {
        if r {
          Ok(m)
        } else {
          bail!("No tokens generated: {:?}", m)
        }
      }, Err(_) => { bail!("Failed to to use Yqcloud now!") }
    }
  }
}

#[cfg(test)]
mod myshell_tests {
  use super::*;
  #[tokio::test]
  async fn myshell_test() {
    let gen = MyshellGenerator;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
