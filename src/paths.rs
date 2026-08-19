use std::env;
use std::path::PathBuf;

/// Platform-aware user locations used by Forge persistent state.
///
/// This module intentionally exposes only paths that Forge currently needs.
/// Configuration and persistent state have separate locations so that one
/// cannot silently become the other as the project grows.
pub fn state_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| env::var_os("APPDATA").map(PathBuf::from))
            .map(|base| base.join("Forge"))
    }

    #[cfg(target_os = "macos")]
    {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join("Library").join("Application Support").join("Forge"))
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .map(|home| PathBuf::from(home).join(".local").join("state"))
            })
            .map(|base| base.join("forge"))
    }
}

/// Platform-aware per-user configuration directory for Forge.
pub fn config_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|base| base.join("Forge"))
    }

    #[cfg(target_os = "macos")]
    {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join("Library").join("Application Support").join("Forge"))
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .map(|home| PathBuf::from(home).join(".config"))
            })
            .map(|base| base.join("forge"))
    }
}

pub fn history_file() -> Option<PathBuf> {
    state_dir().map(|path| path.join("history"))
}

pub fn config_file() -> Option<PathBuf> {
    config_dir().map(|path| path.join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::{config_dir, config_file, state_dir};

    #[test]
    fn state_dir_has_a_platform_specific_base() {
        let path = state_dir();
        if let Some(path) = path {
            assert!(path.ends_with("Forge") || path.ends_with("forge"));
        }
    }

    #[test]
    fn config_dir_has_a_platform_specific_base() {
        let path = config_dir();
        if let Some(path) = path {
            assert!(path.ends_with("Forge") || path.ends_with("forge"));
        }
    }

    #[test]
    fn config_file_is_inside_config_dir() {
        if let (Some(dir), Some(file)) = (config_dir(), config_file()) {
            assert_eq!(file.parent(), Some(dir.as_path()));
            assert_eq!(file.file_name().and_then(|name| name.to_str()), Some("config.toml"));
        }
    }
}
