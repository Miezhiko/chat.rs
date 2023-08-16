import os
import openai

if os.path.isfile("/etc/chat.rs/chimera.txt"):
    file_path = "/etc/chat.rs/chimera.txt"
else:
    file_path = "chimera.txt"
with open(file_path, "r") as file:
  token = file.readline().strip()

openai.api_key = token
openai.api_base = "https://chimeragpt.adventblocks.cc/api/v1"

messages=[
   {'role': 'user', 'content': "привет?"},
]

response = openai.ChatCompletion.create(
  model="llama-2-70b-chat",
  messages=messages,
  stream=False,
  allow_fallback=True
)

choices = response["choices"]
if choices:
  print(choices[0]["message"]["content"])
