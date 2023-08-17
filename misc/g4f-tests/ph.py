import sys
import os
import json
import time
import subprocess

import datetime
import urllib.parse

from curl_cffi import requests

from g4f.typing import sha256, Dict, get_type_hints

url = "https://phind.com"
model = ["gpt-4"]
supports_stream = True

def create_completion(model: str, messages: list, stream: bool):

    path = "/data/contrib/rust/chat.rs/misc"
    config = json.dumps({
        "model": model,
        "messages": messages}, separators=(",", ":"))

    cmd = ["python3", f"{path}/phind.py", config]

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

response = "".join(create_completion(model="gpt-4",
                  stream=False,
                  messages=[{"role": "user", 
                      "content": "I have a nice riddle for you. Can you find the the mistake?\
\
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73"}]
))

print(response)
