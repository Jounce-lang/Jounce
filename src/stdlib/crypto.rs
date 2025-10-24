/// Standard library cryptography module
/// Provides hashing, random generation, encoding, and encryption

pub const CRYPTO_DEFINITION: &str = r#"
// Cryptography Module for Jounce
// Provides security primitives for hashing, random generation, and encoding

// Hash algorithms enum
enum HashAlgorithm {
    SHA256,
    SHA1,
    MD5,
}

// Hash result struct
struct Hash {
    algorithm: HashAlgorithm,
    digest: String,  // Hex-encoded hash
}

impl Hash {
    // Get hash as hex string
    fn to_hex(self: &Hash) -> String {
        return self.digest;
    }

    // Get hash as base64 string
    fn to_base64(self: &Hash) -> String {
        // Convert hex to base64
        // @js: btoa(this.digest.match(/.{2}/g).map(byte => String.fromCharCode(parseInt(byte, 16))).join(''))
        return base64_encode(self.digest);
    }

    // Compare with another hash (constant-time comparison)
    fn eq(self: &Hash, other: &Hash) -> bool {
        if self.digest.len() != other.digest.len() {
            return false;
        }

        let mut result = 0;
        for i in 0..self.digest.len() {
            let a = self.digest.char_at(i);
            let b = other.digest.char_at(i);
            result = result | (a != b);
        }

        return result == 0;
    }
}

// Hashing functions

// SHA-256 hash
fn sha256(data: String) -> Hash {
    // In browser: crypto.subtle.digest('SHA-256', data)
    // In Node.js: crypto.createHash('sha256').update(data).digest('hex')
    // @js_browser: crypto.subtle.digest('SHA-256', new TextEncoder().encode(data)).then(buf => Array.from(new Uint8Array(buf)).map(b => b.toString(16).padStart(2, '0')).join(''))
    // @js_node: require('crypto').createHash('sha256').update(data).digest('hex')

    // Placeholder - will be replaced by JavaScript crypto API
    return Hash {
        algorithm: HashAlgorithm::SHA256,
        digest: "",
    };
}

// SHA-1 hash
fn sha1(data: String) -> Hash {
    // @js_browser: crypto.subtle.digest('SHA-1', new TextEncoder().encode(data))
    // @js_node: require('crypto').createHash('sha1').update(data).digest('hex')

    return Hash {
        algorithm: HashAlgorithm::SHA1,
        digest: "",
    };
}

// MD5 hash (not cryptographically secure - use for checksums only)
fn md5(data: String) -> Hash {
    // @js_node: require('crypto').createHash('md5').update(data).digest('hex')

    return Hash {
        algorithm: HashAlgorithm::MD5,
        digest: "",
    };
}

// Generic hash function
fn hash(algorithm: HashAlgorithm, data: String) -> Hash {
    match algorithm {
        HashAlgorithm::SHA256 => sha256(data),
        HashAlgorithm::SHA1 => sha1(data),
        HashAlgorithm::MD5 => md5(data),
    }
}

// HMAC (Hash-based Message Authentication Code)
fn hmac_sha256(key: String, data: String) -> Hash {
    // @js_node: require('crypto').createHmac('sha256', key).update(data).digest('hex')

    return Hash {
        algorithm: HashAlgorithm::SHA256,
        digest: "",
    };
}

// Random number generation

// Generate secure random bytes
fn random_bytes(length: i32) -> Vec<u8> {
    // @js_browser: crypto.getRandomValues(new Uint8Array(length))
    // @js_node: require('crypto').randomBytes(length)

    let result = Vec::new();
    for i in 0..length {
        result.push(0);
    }
    return result;
}

// Generate secure random integer in range [min, max)
fn random_int(min: i32, max: i32) -> i32 {
    // @js: Math.floor(Math.random() * (max - min)) + min

    let range = max - min;
    if range <= 0 {
        return min;
    }

    // Use secure random if available
    let bytes = random_bytes(4);
    let random_value = (bytes[0] as i32) * 16777216 +
                       (bytes[1] as i32) * 65536 +
                       (bytes[2] as i32) * 256 +
                       (bytes[3] as i32);

    return (random_value % range) + min;
}

// Generate secure random float in range [0, 1)
fn random_float() -> f64 {
    // @js: crypto.getRandomValues(new Uint32Array(1))[0] / 4294967296

    let bytes = random_bytes(4);
    let random_value = (bytes[0] as f64) * 16777216.0 +
                       (bytes[1] as f64) * 65536.0 +
                       (bytes[2] as f64) * 256.0 +
                       (bytes[3] as f64);

    return random_value / 4294967296.0;
}

// Generate random string with specified characters
fn random_string(length: i32, charset: String) -> String {
    let result = "";

    for i in 0..length {
        let index = random_int(0, charset.len());
        result = result + charset.substring(index, index + 1);
    }

    return result;
}

// Generate random alphanumeric string
fn random_alphanumeric(length: i32) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    return random_string(length, charset);
}

// Generate random hex string
fn random_hex(length: i32) -> String {
    let charset = "0123456789abcdef";
    return random_string(length, charset);
}

// UUID generation

// Generate UUID v4 (random)
fn uuid_v4() -> String {
    // Format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    // where x is random hex digit and y is 8, 9, a, or b

    // @js: crypto.randomUUID()

    let hex = random_hex(32);

    // Insert hyphens and version/variant bits
    let uuid = "";
    uuid = uuid + hex.substring(0, 8);
    uuid = uuid + "-";
    uuid = uuid + hex.substring(8, 12);
    uuid = uuid + "-4";  // Version 4
    uuid = uuid + hex.substring(13, 16);
    uuid = uuid + "-";

    // Variant bits (10xx)
    let variant_char = hex.substring(16, 17);
    let variant_value = hex_to_int(variant_char);
    let adjusted = (variant_value & 0x3) | 0x8;
    uuid = uuid + int_to_hex(adjusted);

    uuid = uuid + hex.substring(17, 20);
    uuid = uuid + "-";
    uuid = uuid + hex.substring(20, 32);

    return uuid;
}

// Base64 encoding/decoding

// Encode string to base64
fn base64_encode(data: String) -> String {
    // @js_browser: btoa(data)
    // @js_node: Buffer.from(data).toString('base64')

    // Base64 alphabet
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = "";
    let padding = "";

    // Convert to bytes
    let bytes = Vec::new();
    for i in 0..data.len() {
        bytes.push(data.char_code_at(i));
    }

    // Process in groups of 3 bytes
    let i = 0;
    while i < bytes.len() {
        let b1 = bytes[i];
        let b2 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b3 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        // Convert 3 bytes to 4 base64 characters
        let c1 = b1 >> 2;
        let c2 = ((b1 & 0x03) << 4) | (b2 >> 4);
        let c3 = ((b2 & 0x0F) << 2) | (b3 >> 6);
        let c4 = b3 & 0x3F;

        result = result + alphabet.substring(c1, c1 + 1);
        result = result + alphabet.substring(c2, c2 + 1);

        if i + 1 < bytes.len() {
            result = result + alphabet.substring(c3, c3 + 1);
        } else {
            padding = padding + "=";
        }

        if i + 2 < bytes.len() {
            result = result + alphabet.substring(c4, c4 + 1);
        } else {
            padding = padding + "=";
        }

        i = i + 3;
    }

    return result + padding;
}

// Decode base64 to string
fn base64_decode(encoded: String) -> Result<String, String> {
    // @js_browser: atob(encoded)
    // @js_node: Buffer.from(encoded, 'base64').toString()

    // Remove padding
    let data = encoded.replace("=", "");

    // Base64 alphabet lookup
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let result = "";
    let i = 0;

    while i < data.len() {
        if i + 1 >= data.len() {
            break;
        }

        // Get 4 base64 characters
        let c1 = alphabet.index_of(data.substring(i, i + 1));
        let c2 = alphabet.index_of(data.substring(i + 1, i + 2));
        let c3 = if i + 2 < data.len() { alphabet.index_of(data.substring(i + 2, i + 3)) } else { 0 };
        let c4 = if i + 3 < data.len() { alphabet.index_of(data.substring(i + 3, i + 4)) } else { 0 };

        if c1 < 0 || c2 < 0 {
            return Err("Invalid base64 character");
        }

        // Convert 4 base64 characters to 3 bytes
        let b1 = (c1 << 2) | (c2 >> 4);
        let b2 = ((c2 & 0x0F) << 4) | (c3 >> 2);
        let b3 = ((c3 & 0x03) << 6) | c4;

        result = result + String::from_char_code(b1);

        if i + 2 < data.len() {
            result = result + String::from_char_code(b2);
        }

        if i + 3 < data.len() {
            result = result + String::from_char_code(b3);
        }

        i = i + 4;
    }

    return Ok(result);
}

// Hex encoding/decoding

// Encode bytes to hex string
fn hex_encode(bytes: &[u8]) -> String {
    let hex_chars = "0123456789abcdef";
    let result = "";

    for byte in bytes {
        let high = (byte >> 4) & 0x0F;
        let low = byte & 0x0F;
        result = result + hex_chars.substring(high as i32, (high + 1) as i32);
        result = result + hex_chars.substring(low as i32, (low + 1) as i32);
    }

    return result;
}

// Decode hex string to bytes
fn hex_decode(hex: String) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length");
    }

    let result = Vec::new();
    let i = 0;

    while i < hex.len() {
        let high = hex_char_to_int(hex.substring(i, i + 1));
        let low = hex_char_to_int(hex.substring(i + 1, i + 2));

        if high < 0 || low < 0 {
            return Err("Invalid hex character");
        }

        let byte = (high << 4) | low;
        result.push(byte as u8);

        i = i + 2;
    }

    return Ok(result);
}

// Helper functions

// Convert hex character to integer
fn hex_char_to_int(ch: String) -> i32 {
    if ch >= "0" && ch <= "9" {
        return ch.char_code_at(0) - "0".char_code_at(0);
    } else if ch >= "a" && ch <= "f" {
        return 10 + (ch.char_code_at(0) - "a".char_code_at(0));
    } else if ch >= "A" && ch <= "F" {
        return 10 + (ch.char_code_at(0) - "A".char_code_at(0));
    }
    return -1;
}

// Convert hex string to integer
fn hex_to_int(hex: String) -> i32 {
    let result = 0;
    for i in 0..hex.len() {
        let ch = hex.substring(i, i + 1);
        let val = hex_char_to_int(ch);
        if val < 0 {
            return 0;
        }
        result = result * 16 + val;
    }
    return result;
}

// Convert integer to hex character
fn int_to_hex(value: i32) -> String {
    let hex_chars = "0123456789abcdef";
    return hex_chars.substring(value & 0x0F, (value & 0x0F) + 1);
}

// Password hashing (using PBKDF2)

struct PasswordHash {
    algorithm: String,  // e.g., "PBKDF2-SHA256"
    iterations: i32,
    salt: String,       // Hex-encoded salt
    hash: String,       // Hex-encoded hash
}

impl PasswordHash {
    // Create from components
    fn new(algorithm: String, iterations: i32, salt: String, hash: String) -> PasswordHash {
        return PasswordHash {
            algorithm: algorithm,
            iterations: iterations,
            salt: salt,
            hash: hash,
        };
    }

    // Serialize to string
    fn to_string(self: &PasswordHash) -> String {
        // Format: $algorithm$iterations$salt$hash
        let result = "$";
        result = result + self.algorithm;
        result = result + "$";
        result = result + self.iterations.to_string();
        result = result + "$";
        result = result + self.salt;
        result = result + "$";
        result = result + self.hash;
        return result;
    }

    // Parse from string
    fn from_string(s: String) -> Result<PasswordHash, String> {
        let parts = s.split("$");
        if parts.len() != 5 {
            return Err("Invalid password hash format");
        }

        let algorithm = parts[1];
        let iterations = parts[2].parse_int();
        let salt = parts[3];
        let hash = parts[4];

        return Ok(PasswordHash {
            algorithm: algorithm,
            iterations: iterations,
            salt: salt,
            hash: hash,
        });
    }

    // Verify password
    fn verify(self: &PasswordHash, password: String) -> bool {
        // Hash the password with the same salt and iterations
        let computed = hash_password(password, self.salt, self.iterations);
        return computed.hash == self.hash;
    }
}

// Hash a password with salt
fn hash_password(password: String, salt: String, iterations: i32) -> PasswordHash {
    // Use PBKDF2-HMAC-SHA256
    // @js_node: require('crypto').pbkdf2Sync(password, salt, iterations, 32, 'sha256').toString('hex')

    // Placeholder - will be replaced by JavaScript crypto API
    return PasswordHash {
        algorithm: "PBKDF2-SHA256",
        iterations: iterations,
        salt: salt,
        hash: "",
    };
}

// Generate salt for password hashing
fn generate_salt(length: i32) -> String {
    return random_hex(length);
}

// Convenience function: hash password with auto-generated salt
fn hash_password_auto(password: String) -> PasswordHash {
    let salt = generate_salt(32);  // 32 hex characters = 16 bytes
    let iterations = 100000;       // OWASP recommended minimum
    return hash_password(password, salt, iterations);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_definition_exists() {
        assert!(!CRYPTO_DEFINITION.is_empty());
    }

    #[test]
    fn test_crypto_definition_contains_hashing() {
        assert!(CRYPTO_DEFINITION.contains("fn sha256("));
        assert!(CRYPTO_DEFINITION.contains("fn sha1("));
        assert!(CRYPTO_DEFINITION.contains("fn md5("));
        assert!(CRYPTO_DEFINITION.contains("fn hmac_sha256("));
    }

    #[test]
    fn test_crypto_definition_contains_random() {
        assert!(CRYPTO_DEFINITION.contains("fn random_bytes("));
        assert!(CRYPTO_DEFINITION.contains("fn random_int("));
        assert!(CRYPTO_DEFINITION.contains("fn random_float("));
        assert!(CRYPTO_DEFINITION.contains("fn random_string("));
        assert!(CRYPTO_DEFINITION.contains("fn random_alphanumeric("));
        assert!(CRYPTO_DEFINITION.contains("fn random_hex("));
    }

    #[test]
    fn test_crypto_definition_contains_uuid() {
        assert!(CRYPTO_DEFINITION.contains("fn uuid_v4("));
    }

    #[test]
    fn test_crypto_definition_contains_encoding() {
        assert!(CRYPTO_DEFINITION.contains("fn base64_encode("));
        assert!(CRYPTO_DEFINITION.contains("fn base64_decode("));
        assert!(CRYPTO_DEFINITION.contains("fn hex_encode("));
        assert!(CRYPTO_DEFINITION.contains("fn hex_decode("));
    }

    #[test]
    fn test_crypto_definition_contains_password_hashing() {
        assert!(CRYPTO_DEFINITION.contains("struct PasswordHash"));
        assert!(CRYPTO_DEFINITION.contains("fn hash_password("));
        assert!(CRYPTO_DEFINITION.contains("fn generate_salt("));
        assert!(CRYPTO_DEFINITION.contains("fn hash_password_auto("));
    }

    #[test]
    fn test_crypto_definition_contains_hash_struct() {
        assert!(CRYPTO_DEFINITION.contains("struct Hash"));
        assert!(CRYPTO_DEFINITION.contains("fn to_hex("));
        assert!(CRYPTO_DEFINITION.contains("fn to_base64("));
    }
}
