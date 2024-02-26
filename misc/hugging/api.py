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
  , model="google/gemma-7b-it"
  , max_new_tokens=1000
  , stream=False)

print(output)
