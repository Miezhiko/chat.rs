use crate::personality::get_personality;

use inline_python::{ python, Context };

use std::panic::catch_unwind;
use anyhow::bail;

use chat_utils::help::lang;

pub fn generate( prompt: &str
               , fmode: bool
               , personality: &str
               ) -> anyhow::Result<String> {
  let russian = lang::is_russian(prompt);
  match catch_unwind(|| {
    let c = Context::new();
    c.set("is_russian", russian);
    c.set("fmode", fmode);
    c.set("prompt", prompt);
    c.set("PERSONALITY", get_personality(personality));
    c.run(python! {
      import sys
      import os
      import g4f

      result = ""
      if fmode:
        systemContext = PERSONALITY
      else:
        systemContext = "You are a helpful assistant"
      if is_russian:
        systemContext += ", you reply in Russian, you don't provide Translation"
      else:
        systemContext += ", you reply in English"
      messages = [{"role": "system", "content": systemContext}]
      try:
        messages.append({"role": "user", "content": prompt})
        rspns = g4f.ChatCompletion.create( model=g4f.Model.gpt_4, messages=messages
                                         , stream=False
                                         , provider=g4f.Provider.Forefront )
        if not rspns:
          result = "forefront: Sorry, I can't generate a response right now."
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
    }, Err(_) => { bail!("Failed to to use gpt4free::forefront now!") }
  }
}

#[cfg(test)]
mod ff_tests {
  use super::*;
  #[test]
  fn ff_test() {
    let chat_response =
      generate("what gpt version you use?", true, "Fingon");
    assert!(chat_response.is_ok());
  }
}
