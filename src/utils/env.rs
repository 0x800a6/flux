use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

const INTERNAL_ENV_FILE: &str = ".env";
const ENCRYPTION_KEY: &[u8] = b"flux-shell-secret-key"; // You might want to generate this dynamically

/// Gets the path to the internal environment variable storage file
///
/// # Returns
/// * `PathBuf` - Path to the environment variable file
fn get_internal_env_path() -> PathBuf {
    let mut path: PathBuf = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rip.choco.flux");
    path.push(INTERNAL_ENV_FILE);
    path
}

/// Encodes a value using XOR encryption and base64 encoding
///
/// # Arguments
/// * `value` - String value to encode
///
/// # Returns
/// * Encoded string value
fn encode_value(value: &str) -> String {
    // Simple XOR encryption + base64 encoding
    let encrypted: Vec<u8> = value
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();
    general_purpose::STANDARD.encode(encrypted)
}

/// Decodes an encoded value using XOR decryption and base64 decoding
///
/// # Arguments
/// * `encoded` - Encoded string to decode
///
/// # Returns
/// * `io::Result<String>` - Decoded string or error
fn decode_value(encoded: &str) -> io::Result<String> {
    let encrypted: Vec<u8> = general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let decrypted: Vec<u8> = encrypted
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();

    String::from_utf8(decrypted).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Stores an encrypted environment variable internally
///
/// # Arguments
/// * `key` - Environment variable name
/// * `value` - Value to store
///
/// # Returns
/// * `io::Result<()>` - Success or failure of the operation
pub fn store_internal_env(key: &str, value: &str) -> io::Result<()> {
    let mut vars: HashMap<String, String> = load_internal_envs()?;
    vars.insert(key.to_string(), encode_value(value));
    save_internal_envs(&vars)
}

/// Removes an internal environment variable
///
/// # Arguments
/// * `key` - Name of variable to remove
///
/// # Returns
/// * `io::Result<()>` - Success or failure of the operation
pub fn remove_internal_env(key: &str) -> io::Result<()> {
    let mut vars: HashMap<String, String> = load_internal_envs()?;
    vars.remove(key);
    save_internal_envs(&vars)
}

/// Lists all internal environment variables
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - Map of variable names to values
pub fn list_internal_envs() -> io::Result<HashMap<String, String>> {
    let vars: HashMap<String, String> = load_internal_envs()?;
    let mut decoded: HashMap<String, String> = HashMap::new();

    for (key, encoded_value) in vars {
        if let Ok(value) = decode_value(&encoded_value) {
            decoded.insert(key, value);
        }
    }

    Ok(decoded)
}

/// Loads internal environment variables from storage
///
/// # Returns
/// * `io::Result<HashMap<String, String>>` - Map of variable names to encoded values
fn load_internal_envs() -> io::Result<HashMap<String, String>> {
    let path: PathBuf = get_internal_env_path();

    if !path.exists() {
        return Ok(HashMap::new());
    }

    let mut contents: String = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    Ok(serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new()))
}

/// Saves internal environment variables to storage
///
/// # Arguments
/// * `vars` - Map of variable names to encoded values
///
/// # Returns
/// * `io::Result<()>` - Success or failure of the save operation
fn save_internal_envs(vars: &HashMap<String, String>) -> io::Result<()> {
    let path: PathBuf = get_internal_env_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let contents: String = serde_json::to_string_pretty(vars)?;
    File::create(path)?.write_all(contents.as_bytes())
}

/// Expands environment variables in a string
///
/// Replaces ${VAR} or $VAR with their values from both system
/// and internal environment variables.
///
/// # Arguments
/// * `input` - String containing environment variables to expand
///
/// # Returns
/// * Expanded string with variables replaced by their values
pub fn expand_env_vars(input: &str) -> String {
    let mut result: String = input.to_string();

    if let Ok(internal_vars) = list_internal_envs() {
        for (key, value) in internal_vars {
            result = result
                .replace(&format!("${}", key), &value)
                .replace(&format!("${{{}}}", key), &value);
        }
    }

    for (key, value) in std::env::vars() {
        result = result
            .replace(&format!("${}", key), &value)
            .replace(&format!("${{{}}}", key), &value);
    }

    result
}

/// Sets initial environment variables from configuration
///
/// # Arguments
/// * `vars` - Map of environment variables to set
pub fn set_initial_env_vars(vars: &HashMap<String, String>) {
    for (key, value) in vars {
        std::env::set_var(key, value);
    }

    if std::env::var("SHELL").is_err() {
        if let Ok(exe) = std::env::current_exe() {
            std::env::set_var("SHELL", exe.to_string_lossy().to_string());
        }
    }
}
