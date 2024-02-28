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

output = client.text_generation(
  "Write usage example for std atomic in C++"
  , model="databricks/dolly-v2-12b"
  , max_new_tokens=250
  , stream=False)

print(output)
