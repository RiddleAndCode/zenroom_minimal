keyring = KEYRING.new()
keyring:generate()
print(keyring:public():base64())
print(keyring:private():base64())
