from huggingface_hub import InferenceClient

import os

if os.path.isfile("/etc/chat.rs/hugging.txt"):
    file_path = "/etc/chat.rs/hugging.txt"
else:
    file_path = "hugging.txt"
with open(file_path, "r") as file:
  htoken = file.readline().strip()

client = InferenceClient(token=htoken)

output = None

output = client.conversational("Write usage example for std atomic in C++")

print(output["generated_text"])

if output is not None:
  output = client.conversational("repeat my question"
                                , generated_responses=output["conversation"]["generated_responses"]
                                , past_user_inputs=output["conversation"]["past_user_inputs"]
                                )

print(output["generated_text"])
