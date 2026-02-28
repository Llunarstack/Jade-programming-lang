//! Crypto builtins: sha256, enigma_*, aes_*, password_*, encrypt/decrypt, etc.
//! Delegates to interpreter::crypto.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

fn bytes_from_value(v: &Value) -> Result<Vec<u8>, String> {
    match v {
        Value::String(s) => Ok(s.as_bytes().to_vec()),
        Value::List(items) => items
            .iter()
            .map(|v| match v {
                Value::Integer(i) => Ok(*i as u8),
                _ => Err("Expected list of integers (bytes)".to_string()),
            })
            .collect::<Result<Vec<u8>, String>>(),
        _ => Err("Expected string or byte list".to_string()),
    }
}

fn list_from_bytes(bytes: &[u8]) -> Value {
    Value::List(
        bytes
            .iter()
            .map(|b| Value::Integer(*b as i64))
            .collect(),
    )
}

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
        use crate::interpreter::crypto;
        use sha2::{Digest, Sha256};

        let v = match name {
            "sha256" => {
                if args.len() != 1 {
                    return Err("sha256() takes exactly 1 argument".to_string());
                }
                let val = interpreter.eval_node(&args[0])?;
                let text = match &val {
                    Value::String(s) => s.clone(),
                    _ => return Err("sha256() requires a string argument".to_string()),
                };
                let mut hasher = Sha256::new();
                hasher.update(text.as_bytes());
                let result = hasher.finalize();
                Ok(Some(list_from_bytes(&result)))
            }
            "sha256_hex" => {
                if args.len() != 1 {
                    return Err("sha256_hex() takes exactly 1 argument".to_string());
                }
                let val = interpreter.eval_node(&args[0])?;
                let text = match &val {
                    Value::String(s) => s.clone(),
                    _ => return Err("sha256_hex() requires a string argument".to_string()),
                };
                let mut hasher = Sha256::new();
                hasher.update(text.as_bytes());
                let result = hasher.finalize();
                let hex_string = result.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                Ok(Some(Value::String(hex_string)))
            }
            "hmac" => {
                if args.len() != 2 {
                    return Err("hmac() takes exactly 2 arguments: (message, key)".to_string());
                }
                let message_val = interpreter.eval_node(&args[0])?;
                let key_val = interpreter.eval_node(&args[1])?;
                let message = match &message_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("hmac() message must be a string".to_string()),
                };
                let key = match &key_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("hmac() key must be a string".to_string()),
                };
                use hmac::{Hmac, Mac};
                type HmacSha256 = Hmac<Sha256>;
                let mut mac = HmacSha256::new_from_slice(key.as_bytes())
                    .map_err(|e| format!("HMAC error: {}", e))?;
                mac.update(message.as_bytes());
                let result = mac.finalize();
                Ok(Some(list_from_bytes(result.into_bytes().as_slice())))
            }
            "secure_eq" => {
                if args.len() != 2 {
                    return Err("secure_eq() takes exactly 2 arguments".to_string());
                }
                let a_val = interpreter.eval_node(&args[0])?;
                let b_val = interpreter.eval_node(&args[1])?;
                let a = match &a_val {
                    Value::String(s) => s.as_str(),
                    _ => return Err("secure_eq() requires string arguments".to_string()),
                };
                let b = match &b_val {
                    Value::String(s) => s.as_str(),
                    _ => return Err("secure_eq() requires string arguments".to_string()),
                };
                let mut result = 0u8;
                for (x, y) in a.bytes().zip(b.bytes()) {
                    result |= x ^ y;
                }
                Ok(Some(Value::Boolean(result == 0 && a.len() == b.len())))
            }
            "enigma_keypair" => {
                if !args.is_empty() {
                    return Err("enigma_keypair() takes no arguments".to_string());
                }
                let _kp = crypto::EnigmaKeypair::generate();
                Ok(Some(Value::String("EnigmaKeypair(generated)".to_string())))
            }
            "enigma_encrypt" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err(
                        "enigma_encrypt() takes 3-4 arguments: (plaintext, key, nonce, [aad])"
                            .to_string(),
                    );
                }
                let plaintext_val = interpreter.eval_node(&args[0])?;
                let key_val = interpreter.eval_node(&args[1])?;
                let nonce_val = interpreter.eval_node(&args[2])?;
                let aad_val = if args.len() == 4 {
                    interpreter.eval_node(&args[3])?
                } else {
                    Value::String(String::new())
                };
                let plaintext = match &plaintext_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    Value::List(items) => bytes_from_value(&Value::List(items.clone()))?,
                    _ => return Err("enigma_encrypt() plaintext must be string or byte list".to_string()),
                };
                let key = bytes_from_value(&key_val).map_err(|_| "enigma_encrypt() key must be a byte list".to_string())?;
                let nonce = bytes_from_value(&nonce_val).map_err(|_| "enigma_encrypt() nonce must be a byte list".to_string())?;
                let aad = match &aad_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    Value::List(items) => bytes_from_value(&Value::List(items.clone())).unwrap_or_default(),
                    _ => vec![],
                };
                let ciphertext = crypto::enigma_encrypt(&plaintext, &key, &nonce, &aad)?;
                Ok(Some(list_from_bytes(&ciphertext)))
            }
            "enigma_decrypt" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err(
                        "enigma_decrypt() takes 3-4 arguments: (ciphertext, key, nonce, [aad])"
                            .to_string(),
                    );
                }
                let ciphertext_val = interpreter.eval_node(&args[0])?;
                let key_val = interpreter.eval_node(&args[1])?;
                let nonce_val = interpreter.eval_node(&args[2])?;
                let aad_val = if args.len() == 4 {
                    interpreter.eval_node(&args[3])?
                } else {
                    Value::String(String::new())
                };
                let ciphertext = bytes_from_value(&ciphertext_val)
                    .map_err(|_| "enigma_decrypt() ciphertext must be a byte list".to_string())?;
                let key = bytes_from_value(&key_val).map_err(|_| "enigma_decrypt() key must be a byte list".to_string())?;
                let nonce = bytes_from_value(&nonce_val).map_err(|_| "enigma_decrypt() nonce must be a byte list".to_string())?;
                let aad = match &aad_val {
                    Value::String(s) => s.as_bytes().to_vec(),
                    Value::List(items) => bytes_from_value(&Value::List(items.clone())).unwrap_or_default(),
                    _ => vec![],
                };
                let plaintext = crypto::enigma_decrypt(&ciphertext, &key, &nonce, &aad)?;
                match String::from_utf8(plaintext.clone()) {
                    Ok(s) => Ok(Some(Value::String(s))),
                    Err(_) => Ok(Some(list_from_bytes(&plaintext))),
                }
            }
            "crypto_random_bytes" => {
                if args.len() != 1 {
                    return Err("crypto_random_bytes() takes exactly 1 argument (length)".to_string());
                }
                let len_val = interpreter.eval_node(&args[0])?;
                let len = match &len_val {
                    Value::Integer(n) if *n > 0 => *n as usize,
                    _ => return Err("crypto_random_bytes() requires a positive integer".to_string()),
                };
                let bytes = crypto::random_bytes(len);
                Ok(Some(list_from_bytes(&bytes)))
            }
            "xor_bytes" => {
                if args.len() != 2 {
                    return Err("xor_bytes() takes exactly 2 arguments (bytes_a, bytes_b)".to_string());
                }
                let a_val = interpreter.eval_node(&args[0])?;
                let b_val = interpreter.eval_node(&args[1])?;
                let a = bytes_from_value(&a_val)
                    .map_err(|_| "xor_bytes() arguments must be byte lists or strings".to_string())?;
                let b = bytes_from_value(&b_val)
                    .map_err(|_| "xor_bytes() arguments must be byte lists or strings".to_string())?;
                let result = crypto::xor_bytes(&a, &b)?;
                Ok(Some(list_from_bytes(&result)))
            }
            "aes_encrypt" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err(
                        "aes_encrypt(plaintext, key, nonce, [aad]) requires 3-4 arguments".to_string(),
                    );
                }
                let plaintext = bytes_from_value(&interpreter.eval_node(&args[0])?)
                    .map_err(|_| "aes_encrypt plaintext must be string or byte list".to_string())?;
                let key = bytes_from_value(&interpreter.eval_node(&args[1])?)
                    .map_err(|_| "aes_encrypt key must be byte list".to_string())?;
                let nonce = bytes_from_value(&interpreter.eval_node(&args[2])?)
                    .map_err(|_| "aes_encrypt nonce must be byte list".to_string())?;
                let aad = if args.len() == 4 {
                    bytes_from_value(&interpreter.eval_node(&args[3])?).unwrap_or_default()
                } else {
                    vec![]
                };
                let ciphertext = crypto::aes_encrypt(&plaintext, &key, &nonce, &aad)?;
                Ok(Some(list_from_bytes(&ciphertext)))
            }
            "aes_decrypt" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err(
                        "aes_decrypt(ciphertext, key, nonce, [aad]) requires 3-4 arguments".to_string(),
                    );
                }
                let ciphertext = bytes_from_value(&interpreter.eval_node(&args[0])?)
                    .map_err(|_| "aes_decrypt ciphertext must be byte list".to_string())?;
                let key = bytes_from_value(&interpreter.eval_node(&args[1])?)
                    .map_err(|_| "aes_decrypt key must be byte list".to_string())?;
                let nonce = bytes_from_value(&interpreter.eval_node(&args[2])?)
                    .map_err(|_| "aes_decrypt nonce must be byte list".to_string())?;
                let aad = if args.len() == 4 {
                    bytes_from_value(&interpreter.eval_node(&args[3])?).unwrap_or_default()
                } else {
                    vec![]
                };
                let plaintext = crypto::aes_decrypt(&ciphertext, &key, &nonce, &aad)?;
                match String::from_utf8(plaintext.clone()) {
                    Ok(s) => Ok(Some(Value::String(s))),
                    Err(_) => Ok(Some(list_from_bytes(&plaintext))),
                }
            }
            "derive_password_key" => {
                if args.len() < 2 || args.len() > 4 {
                    return Err(
                        "derive_password_key(password, salt, [ops_limit], [mem_limit_kb]) requires 2-4 arguments".to_string(),
                    );
                }
                let password_val = interpreter.eval_node(&args[0])?;
                let password = match &password_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("derive_password_key password must be string".to_string()),
                };
                let salt = bytes_from_value(&interpreter.eval_node(&args[1])?)
                    .map_err(|_| "derive_password_key salt must be byte list".to_string())?;
                let ops_limit = if args.len() >= 3 {
                    match &interpreter.eval_node(&args[2])? {
                        Value::Integer(i) if *i > 0 => *i as u32,
                        _ => return Err("derive_password_key ops_limit must be positive integer".to_string()),
                    }
                } else {
                    2
                };
                let mem_limit_kb = if args.len() >= 4 {
                    match &interpreter.eval_node(&args[3])? {
                        Value::Integer(i) if *i > 0 => *i as u32,
                        _ => return Err("derive_password_key mem_limit_kb must be positive integer".to_string()),
                    }
                } else {
                    19456
                };
                let key = crypto::derive_password_key(&password, &salt, ops_limit, mem_limit_kb)?;
                Ok(Some(list_from_bytes(&key)))
            }
            "crypto_salt" => {
                let length = if args.is_empty() {
                    16
                } else if args.len() == 1 {
                    match &interpreter.eval_node(&args[0])? {
                        Value::Integer(i) if *i > 0 => *i as usize,
                        _ => return Err("crypto_salt([length]) length must be positive integer".to_string()),
                    }
                } else {
                    return Err("crypto_salt([length]) takes 0-1 arguments".to_string());
                };
                let salt = crypto::generate_salt(length);
                Ok(Some(list_from_bytes(&salt)))
            }
            "crypto_nonce" => {
                let length = if args.is_empty() {
                    12
                } else if args.len() == 1 {
                    match &interpreter.eval_node(&args[0])? {
                        Value::Integer(i) if *i > 0 => *i as usize,
                        _ => return Err("crypto_nonce([length]) length must be positive integer".to_string()),
                    }
                } else {
                    return Err("crypto_nonce([length]) takes 0-1 arguments".to_string());
                };
                let nonce = crypto::generate_nonce(length);
                Ok(Some(list_from_bytes(&nonce)))
            }
            "secure_compare" => {
                if args.len() != 2 {
                    return Err("secure_compare(a, b) requires 2 arguments".to_string());
                }
                let a = bytes_from_value(&interpreter.eval_node(&args[0])?)?;
                let b = bytes_from_value(&interpreter.eval_node(&args[1])?)?;
                Ok(Some(Value::Boolean(crypto::secure_compare(&a, &b))))
            }
            "encrypt_value" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(
                        "encrypt_value(value, key_id, [key]) requires 2-3 arguments".to_string(),
                    );
                }
                let value_to_encrypt = interpreter.eval_node(&args[0])?;
                let key_id = match &interpreter.eval_node(&args[1])? {
                    Value::String(s) => s.clone(),
                    _ => return Err("key_id must be a string".to_string()),
                };
                let key = if args.len() == 3 {
                    let key_val = interpreter.eval_node(&args[2])?;
                    match &key_val {
                        Value::List(_) => bytes_from_value(&key_val)?,
                        Value::String(s) => {
                            let salt = crypto::generate_salt(16);
                            crypto::derive_password_key(s, &salt, 2, 19456)?
                        }
                        _ => return Err("Key must be a byte list or password string".to_string()),
                    }
                } else {
                    let mut hasher = Sha256::new();
                    hasher.update(key_id.as_bytes());
                    hasher.finalize().to_vec()
                };
                let json_str = match &value_to_encrypt {
                    Value::Integer(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::String(s) => format!("\"{}\"", s),
                    Value::Boolean(b) => b.to_string(),
                    _ => format!("{:?}", value_to_encrypt),
                };
                let nonce = crypto::generate_nonce(12);
                let ciphertext = crypto::enigma_encrypt(json_str.as_bytes(), &key, &nonce, b"")?;
                Ok(Some(Value::Encrypted {
                    ciphertext,
                    key_id,
                    nonce,
                }))
            }
            "decrypt_value" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(
                        "decrypt_value(encrypted_value, [key]) requires 1-2 arguments".to_string(),
                    );
                }
                let encrypted_val = interpreter.eval_node(&args[0])?;
                let (ciphertext, key_id, nonce) = match &encrypted_val {
                    Value::Encrypted {
                        ciphertext,
                        key_id,
                        nonce,
                    } => (ciphertext.clone(), key_id.clone(), nonce.clone()),
                    _ => return Err("decrypt_value() requires an encrypted value".to_string()),
                };
                let key = if args.len() == 2 {
                    let key_val = interpreter.eval_node(&args[1])?;
                    match &key_val {
                        Value::List(_) => bytes_from_value(&key_val)?,
                        Value::String(s) => {
                            let salt = crypto::generate_salt(16);
                            crypto::derive_password_key(s, &salt, 2, 19456)?
                        }
                        _ => return Err("Key must be a byte list or password string".to_string()),
                    }
                } else {
                    let mut hasher = Sha256::new();
                    hasher.update(key_id.as_bytes());
                    hasher.finalize().to_vec()
                };
                let plaintext_bytes = crypto::enigma_decrypt(&ciphertext, &key, &nonce, b"")?;
                let plaintext = String::from_utf8_lossy(&plaintext_bytes).to_string();
                let out = if let Ok(i) = plaintext.parse::<i64>() {
                    Value::Integer(i)
                } else if let Ok(f) = plaintext.parse::<f64>() {
                    Value::Float(f)
                } else if plaintext == "true" {
                    Value::Boolean(true)
                } else if plaintext == "false" {
                    Value::Boolean(false)
                } else if plaintext.starts_with('"') && plaintext.ends_with('"') {
                    Value::String(plaintext[1..plaintext.len() - 1].to_string())
                } else {
                    Value::String(plaintext)
                };
                Ok(Some(out))
            }
            "password_hash" => {
                if args.len() != 1 {
                    return Err("password_hash() takes exactly 1 argument".to_string());
                }
                let password_val = interpreter.eval_node(&args[0])?;
                let password = match &password_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("password_hash() requires a string argument".to_string()),
                };
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let salt: String = (0..16).map(|_| format!("{:02x}", rng.gen::<u8>())).collect();
                let mut hasher = Sha256::new();
                hasher.update(salt.as_bytes());
                hasher.update(password.as_bytes());
                let hash = hasher.finalize();
                let hash_hex = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                Ok(Some(Value::String(format!("${}${}", salt, hash_hex))))
            }
            "password_verify" => {
                if args.len() != 2 {
                    return Err(
                        "password_verify() takes exactly 2 arguments (password, hash)".to_string(),
                    );
                }
                let password_val = interpreter.eval_node(&args[0])?;
                let hash_val = interpreter.eval_node(&args[1])?;
                let password = match &password_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("password_verify() requires string arguments".to_string()),
                };
                let stored_hash = match &hash_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("password_verify() requires string arguments".to_string()),
                };
                let parts: Vec<&str> = stored_hash.split('$').collect();
                if parts.len() != 3 || !parts[0].is_empty() {
                    return Err("Invalid password hash format".to_string());
                }
                let salt = parts[1];
                let expected_hash = parts[2];
                let mut hasher = Sha256::new();
                hasher.update(salt.as_bytes());
                hasher.update(password.as_bytes());
                let hash = hasher.finalize();
                let computed_hash = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                let mut result = 0u8;
                for (x, y) in computed_hash.bytes().zip(expected_hash.bytes()) {
                    result |= x ^ y;
                }
                Ok(Some(Value::Boolean(
                    result == 0 && computed_hash.len() == expected_hash.len(),
                )))
            }
            "encrypt" => {
                if args.len() != 2 {
                    return Err("encrypt() takes exactly 2 arguments (plaintext, key)".to_string());
                }
                let plaintext_val = interpreter.eval_node(&args[0])?;
                let key_val = interpreter.eval_node(&args[1])?;
                let plaintext = match &plaintext_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("encrypt() plaintext must be a string".to_string()),
                };
                let key = match &key_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("encrypt() key must be a string".to_string()),
                };
                let mut hasher = Sha256::new();
                hasher.update(key.as_bytes());
                let key_hash = hasher.finalize();
                let encrypted: Vec<u8> = plaintext
                    .bytes()
                    .enumerate()
                    .map(|(i, b)| b ^ key_hash[i % key_hash.len()])
                    .collect();
                use base64::{engine::general_purpose, Engine as _};
                Ok(Some(Value::String(general_purpose::STANDARD.encode(&encrypted))))
            }
            "decrypt" => {
                if args.len() != 2 {
                    return Err("decrypt() takes exactly 2 arguments (ciphertext, key)".to_string());
                }
                let ciphertext_val = interpreter.eval_node(&args[0])?;
                let key_val = interpreter.eval_node(&args[1])?;
                let ciphertext = match &ciphertext_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("decrypt() ciphertext must be a string".to_string()),
                };
                let key = match &key_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("decrypt() key must be a string".to_string()),
                };
                use base64::{engine::general_purpose, Engine as _};
                let encrypted = general_purpose::STANDARD
                    .decode(&ciphertext)
                    .map_err(|e| format!("decrypt() invalid base64: {}", e))?;
                let mut hasher = Sha256::new();
                hasher.update(key.as_bytes());
                let key_hash = hasher.finalize();
                let decrypted: Vec<u8> = encrypted
                    .iter()
                    .enumerate()
                    .map(|(i, b)| b ^ key_hash[i % key_hash.len()])
                    .collect();
                let plaintext = String::from_utf8(decrypted)
                    .map_err(|e| format!("decrypt() invalid UTF-8: {}", e))?;
                Ok(Some(Value::String(plaintext)))
            }
            "secure_token" => {
                if args.len() != 1 {
                    return Err("secure_token() takes exactly 1 argument (length)".to_string());
                }
                let len_val = interpreter.eval_node(&args[0])?;
                let len = match &len_val {
                    Value::Integer(n) if *n > 0 => *n as usize,
                    _ => return Err("secure_token() requires a positive integer".to_string()),
                };
                use rand::Rng;
                const CHARSET: &[u8] =
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                let mut rng = rand::thread_rng();
                let token: String = (0..len)
                    .map(|_| {
                        let idx = rng.gen_range(0..CHARSET.len());
                        CHARSET[idx] as char
                    })
                    .collect();
                Ok(Some(Value::String(token)))
            }
            _ => return Ok(None),
        };
        v
}
