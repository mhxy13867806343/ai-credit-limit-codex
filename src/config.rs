use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub refresh_interval_secs: u64,
    #[allow(dead_code)]
    pub custom_codex_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_interval_secs: 120, // 2 minutes
            custom_codex_path: None,
        }
    }
}

/// Helper function to locate the `codex` CLI executable on macOS
pub fn find_codex_bin() -> Option<PathBuf> {
    // 1. Check if user configured a custom path or environment variable
    if let Ok(env_path) = std::env::var("CODEX_PATH") {
        let p = PathBuf::from(env_path);
        if p.exists() {
            return Some(p);
        }
    }

    // 2. Check standard PATH
    if let Ok(output) = std::process::Command::new("which").arg("codex").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let p = PathBuf::from(path_str);
                if p.exists() {
                    return Some(p);
                }
            }
        }
    }

    // 3. Check common node / package manager locations on macOS
    if let Ok(home) = std::env::var("HOME") {
        let candidates = [
            format!("{}/n/bin/codex", home),
            format!("{}/.nvm/versions/node/current/bin/codex", home),
            format!("{}/.bun/bin/codex", home),
            format!("{}/.cargo/bin/codex", home),
            "/usr/local/bin/codex".to_string(),
            "/opt/homebrew/bin/codex".to_string(),
            "/opt/homebrew/default/bin/codex".to_string(),
        ];

        for c in candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

/// Helper function to get ~/.codex directory
pub fn get_codex_home_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("CODEX_HOME") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return Some(p);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(".codex");
        if p.exists() {
            return Some(p);
        }
    }

    None
}
