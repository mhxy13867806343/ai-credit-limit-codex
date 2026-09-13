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

/// Enrich process PATH with user environment and common directories on macOS
pub fn ensure_gui_app_path() {
    let mut extra = vec![
        "/opt/homebrew/bin".to_string(),
        "/opt/homebrew/sbin".to_string(),
        "/usr/local/bin".to_string(),
        "/usr/local/sbin".to_string(),
    ];
    if let Ok(home) = std::env::var("HOME") {
        extra.push(format!("{}/n/bin", home));
        extra.push(format!("{}/.bun/bin", home));
        extra.push(format!("{}/.cargo/bin", home));
        extra.push(format!("{}/.local/bin", home));
        extra.push(format!("{}/.volta/bin", home));
        let nvm = PathBuf::from(&home).join(".nvm/versions/node");
        if let Ok(entries) = std::fs::read_dir(nvm) {
            for entry in entries.flatten() {
                let bin = entry.path().join("bin");
                if bin.exists() { extra.push(bin.to_string_lossy().to_string()); }
            }
        }
    }
    let cur = std::env::var("PATH").unwrap_or_default();
    let mut parts = extra;
    parts.push(cur);
    std::env::set_var("PATH", parts.join(":"));
}

/// Helper function to locate the `codex` CLI executable on macOS
pub fn find_codex_bin() -> Option<PathBuf> {
    if let Ok(env_path) = std::env::var("CODEX_PATH") {
        let p = PathBuf::from(env_path);
        if p.exists() { return Some(p); }
    }

    if let Ok(home) = std::env::var("HOME") {
        let native_candidates = [
            format!("{}/n/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex", home),
            format!("{}/n/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-x64/vendor/x86_64-apple-darwin/bin/codex", home),
            format!("{}/n/lib/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex", home),
            format!("{}/.bun/install/global/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex", home),
            "/opt/homebrew/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex".to_string(),
            "/usr/local/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex".to_string(),
        ];
        for c in native_candidates {
            let p = PathBuf::from(c);
            if p.exists() { return Some(p); }
        }

        let cli_candidates = [
            format!("{}/n/bin/codex", home),
            format!("{}/.bun/bin/codex", home),
            format!("{}/.cargo/bin/codex", home),
            "/opt/homebrew/bin/codex".to_string(),
            "/usr/local/bin/codex".to_string(),
        ];
        for c in cli_candidates {
            let p = PathBuf::from(c);
            if p.exists() { return Some(p); }
        }
    }

    if let Ok(output) = std::process::Command::new("which").arg("codex").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let p = PathBuf::from(path_str);
                if p.exists() { return Some(p); }
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

/// Check if Codex desktop application is installed
pub fn is_codex_desktop_app_installed() -> bool {
    let candidates = [
        "/Applications/Codex.app",
        "/Applications/ChatGPT.app",
    ];
    for c in candidates {
        if std::path::Path::new(c).exists() {
            return true;
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let user_app = format!("{}/Applications/Codex.app", home);
        if std::path::Path::new(&user_app).exists() {
            return true;
        }
    }
    false
}

/// Check if Codex CLI executable is installed
pub fn is_codex_cli_installed() -> bool {
    find_codex_bin().is_some()
}

/// Check if any Codex environment (App or CLI) is installed
pub fn is_codex_installed() -> bool {
    is_codex_desktop_app_installed() || is_codex_cli_installed()
}
