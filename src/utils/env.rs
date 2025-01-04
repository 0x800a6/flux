use std::collections::HashMap;
use std::env;

pub fn expand_env_vars(input: &str) -> String {
    let mut result: String = input.to_string();
    for (key, value) in env::vars() {
        result = result
            .replace(&format!("${}", key), &value)
            .replace(&format!("${{{}}}", key), &value);
    }
    result
}

pub fn set_initial_env_vars(vars: &HashMap<String, String>) {
    for (key, value) in vars {
        env::set_var(key, value);
    }

    // Set default shell environment variable
    if env::var("SHELL").is_err() {
        if let Ok(exe) = env::current_exe() {
            env::set_var("SHELL", exe.to_string_lossy().to_string());
        }
    }
}
