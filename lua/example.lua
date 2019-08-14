keyring = KEYRING.new()
keyring:generate()
message = OCTET.string("hello")
signature = keyring:sign(message)
print(keyring:verify(message, signature))
