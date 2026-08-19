use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;

use crate::paths;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiConfig {
    /// Prompt template. `{cwd}` is replaced with the current working directory.
    pub prompt: Option<String>,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let Some(path) = paths::config_file() else {
            return Ok(Self::default());
        };
        Self::load_from_path(&path)
    }

    /// Load a configuration from an explicit path.
    ///
    /// Keeping file I/O injectable makes configuration parsing deterministic
    /// in tests without mutating the user's real environment.
    pub fn load_from_path(path: &Path) -> io::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot read configuration '{}': {error}", path.display()),
            )
        })?;

        toml::from_str(&content).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid configuration '{}': {error}", path.display()),
            )
        })
    }

    pub fn prompt(&self, current_dir: &Path) -> String {
        self.ui
            .prompt
            .as_deref()
            .unwrap_or("forge {cwd}> ")
            .replace("{cwd}", &current_dir.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn defaults_to_the_standard_prompt() {
        let config = Config::default();
        assert_eq!(config.prompt(Path::new("/tmp")), "forge /tmp> ");
    }

    #[test]
    fn expands_current_directory_in_custom_prompt() {
        let config: Config = toml::from_str("[ui]\nprompt = \"FORGE {cwd} :: \"").unwrap();
        assert_eq!(config.prompt(Path::new("/tmp/project")), "FORGE /tmp/project :: ");
    }

    #[test]
    fn rejects_unknown_settings_without_silently_ignoring_them() {
        let result: Result<Config, _> = toml::from_str("[ui]\nunknown = true");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_setting_types_without_panicking() {
        let result: Result<Config, _> = toml::from_str("[ui]\nprompt = [\"not a string\"]");
        assert!(result.is_err());
    }

    #[test]
    fn prompt_replaces_all_cwd_placeholders() {
        let config: Config = toml::from_str("[ui]\nprompt = \"{cwd} | {cwd}\"").unwrap();
        assert_eq!(config.prompt(Path::new("/tmp/project")), "/tmp/project | /tmp/project");
    }

    #[test]
    fn prompt_treats_unknown_braced_text_as_literal() {
        let config: Config = toml::from_str("[ui]\nprompt = \"forge {unknown} {cwd}> \"").unwrap();
        assert_eq!(config.prompt(Path::new("/tmp/project")), "forge {unknown} /tmp/project> ");
    }

    #[test]
    fn missing_config_file_returns_defaults() {
        let path = unique_temp_path("forge-config-missing");
        let config = Config::load_from_path(&path).unwrap();
        assert_eq!(config.prompt(Path::new("/tmp")), "forge /tmp> ");
    }

    #[test]
    fn loads_valid_config_from_an_explicit_path() {
        let path = unique_temp_path("forge-config-valid");
        fs::write(&path, "[ui]\nprompt = \"FORGE {cwd} :: \"").unwrap();

        let config = Config::load_from_path(&path).unwrap();
        assert_eq!(config.prompt(Path::new("/tmp/project")), "FORGE /tmp/project :: ");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn reports_invalid_toml_as_invalid_data() {
        let path = unique_temp_path("forge-config-invalid");
        fs::write(&path, "[ui\nprompt = \"broken\"").unwrap();

        let error = Config::load_from_path(&path).expect_err("invalid TOML must fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("invalid configuration"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn reports_unknown_fields_when_loading_from_file() {
        let path = unique_temp_path("forge-config-unknown");
        fs::write(&path, "[ui]\nunknown = true").unwrap();

        let error = Config::load_from_path(&path).expect_err("unknown fields must fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        fs::remove_file(path).unwrap();
    }

    fn unique_temp_path(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{}-{nonce}.toml", std::process::id()))
    }
}
