import g4f

# normal response
response = g4f.ChatCompletion.create( model=g4f.models.gpt_4, messages=[
                                     {"role": "user", "content": "write a poem about mind flayers"}]
                                     , stream=False, auth='jwt'
                                     , provider=g4f.Provider.EasyChat )

print(response)
