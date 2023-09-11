use crate::types::Generator;

use inline_python::{ python, Context };

use std::panic::catch_unwind;

use async_trait::async_trait;

use anyhow::bail;

pub struct PhindGenerator;

#[async_trait]
impl Generator for PhindGenerator {
  fn name<'a>( &self ) -> &'a str {
    "Phind"
  }
  async fn call(&self, prompt: &str, fmode: bool, personality: &str)
    -> anyhow::Result<String> {
    generate(prompt, fmode, personality).await
  }
}

// TODO: pass personality
// TODO: fix hardcoded phind.py path
// TODO: refactor python code
async fn generate( prompt: &str
                 , _fmode: bool
                 , _personality: &str
                 ) -> anyhow::Result<String> {
  match catch_unwind(|| {
    let c = Context::new();
    c.set("prompt", prompt);
    c.run(python! {
      import sys
      import os
      import json
      import time
      import subprocess
      import datetime
      import urllib.parse
      
      from curl_cffi import requests

      url = "https://phind.com"
      model = ["gpt-4"]
      supports_stream = False
      
      def create_completion(model: str, messages: list, stream: bool):
      
          config = json.dumps({
              "model": model,
              "messages": messages}, separators=(",", ":"))
      
          cmd = ["python3", "/data/contrib/rust/chat.rs/misc/phind.py", config]

          p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
      
          for line in iter(p.stdout.readline, b""):
              if b"<title>Just a moment...</title>" in line:
                  os.system("clear" if os.name == "posix" else "cls")
                  yield "Clouflare error, please try again..."
                  os._exit(0)
              else:
                  if b"ping - 2023-" in line:
                      continue
                  
                  yield line.decode("utf-8") 

      result = ""
      try:
        messages = [{"role": "user", "content": prompt}]
        rspns = "".join(create_completion(model="gpt-4",
                        messages=messages, 
                        stream=False))
        if not rspns:
          result = "phind: Sorry, I can't generate a response right now."
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
    }, Err(_) => { bail!("Failed to to use phind now!") }
  }
}

#[cfg(test)]
mod phind_tests {
  use super::*;
  #[tokio::test]
  async fn phind_test() {
    let chat_response =
      generate("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
