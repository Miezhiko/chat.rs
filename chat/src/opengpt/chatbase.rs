use inline_python::{ python, Context };

use std::panic::catch_unwind;

use anyhow::bail;

use chat_utils::help::lang;

pub fn generate(prompt: &str) -> anyhow::Result<String> {
  let russian = lang::is_russian(prompt);
  match catch_unwind(|| {
    let c = Context::new();
    c.set("prompt", prompt);
    c.set("is_russian", russian);
    c.run(python! {
      import sys
      import os

      from opengpt.models.completion.chatbase.model import Model

      if is_russian:
        prompt += ", you reply only in Russian"
      result = ""
      try:
        chatbase = Model()
        rspns = chatbase.GetAnswer(prompt=prompt, model="gpt-4")
        if not rspns:
          result = "chatbase: Sorry, I can't generate a response right now."
          reslt = False
        else:
          reslt = True
          for token in rspns:
            result += token
            result = result.replace("DAN Mode enabled.\n\n", "")
            result = result.replace("DAN Mode enabled\n\n", "")
            result = result.replace("DAN Mode enabled", "")
            result = result.replace("DAN: ", "")
            result = result.replace("GPT: ", "")
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
        if m.contains("Please reduce the length of the messages or completion") {
          bail!("No enough tokens! {:?}", m)
        } else {
          Ok(m)
        }
      } else {
        bail!("No tokens generated: {:?}", m)
      }
    }, Err(_) => { bail!("Failed to to use opengpt::chatbase now!") }
  }
}

#[cfg(test)]
mod chatbase_tests {
  use super::*;
  #[test]
  fn chatbase_test() {
    let chat_response =
      generate("what gpt version you use?");
    assert!(chat_response.is_ok());
  }
}
