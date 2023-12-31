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
   {'role': 'user', 'content': "I have a nice riddle for you. Can you find the the mistake?\
\
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73"},
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
