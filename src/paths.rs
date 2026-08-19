use std::env;
use std::path::PathBuf;

/// Platform-aware user locations used by Forge persistent state.
///
/// This module intentionally exposes only paths that Forge currently needs.
/// A future configuration subsystem can build on this boundary without
/// coupling the shell to environment-variable conventions.
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

pub fn history_file() -> Option<PathBuf> {
    state_dir().map(|path| path.join("history"))
}

#[cfg(test)]
mod tests {
    use super::state_dir;

    #[test]
    fn state_dir_has_a_platform_specific_base() {
        let path = state_dir();
        if let Some(path) = path {
            assert!(path.ends_with("Forge") || path.ends_with("forge"));
        }
    }
}
