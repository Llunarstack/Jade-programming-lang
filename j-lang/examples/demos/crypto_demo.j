# Cryptography & Security Demo

out("=== Cryptography & Security Features ===")
out("")

# SHA256 Hashing
out("1. SHA256 Hashing")
str | hash_hello -> sha256_hex("hello")
str | hash_world -> sha256_hex("world")
out(hash_hello)
out(hash_world)
out("")

# Random Numbers
out("2. Random Numbers (1-100)")
int | rand1 -> rand_range(1, 100)
int | rand2 -> rand_range(1, 100)
int | rand3 -> rand_range(1, 100)
out(rand1)
out(rand2)
out(rand3)
out("")

# UUID Generation
out("3. UUID v4")
str | uuid1 -> uuid_v4()
str | uuid2 -> uuid_v4()
out(uuid1)
out(uuid2)
out("")

# Secure Tokens
out("4. Secure Tokens")
str | token16 -> secure_token(16)
str | token32 -> secure_token(32)
out(token16)
out(token32)
out("")

# Password Hashing
out("5. Password Hashing & Verification")
str | mypass -> "SecurePassword123"
str | hashed_pass -> password_hash(mypass)
out(hashed_pass)

bool | verify_ok -> password_verify(mypass, hashed_pass)
bool | verify_bad -> password_verify("WrongPass", hashed_pass)
out(verify_ok)
out(verify_bad)
out("")

# Encryption
out("6. Encryption & Decryption")
str | plain -> "Secret Message"
str | enc_key -> "MyEncryptionKey"
str | cipher -> encrypt(plain, enc_key)
str | plain2 -> decrypt(cipher, enc_key)
out(cipher)
out(plain2)
bool | same -> plain == plain2
out(same)
out("")

# Secure Equality
out("7. Secure Equality (constant-time)")
bool | eq_true -> secure_eq("test", "test")
bool | eq_false -> secure_eq("test", "fail")
out(eq_true)
out(eq_false)
out("")

# Random Bytes
out("8. Random Bytes")
list | bytes8 -> random_bytes(8)
list | bytes16 -> random_bytes(16)
out(len(bytes8))
out(len(bytes16))
out("")

# HMAC
out("9. HMAC")
str | msg -> "important data"
str | hmac_key -> "shared_key"
list | mac_result -> hmac(msg, hmac_key)
out(len(mac_result))
out("")

out("=== All Tests Complete ===")
