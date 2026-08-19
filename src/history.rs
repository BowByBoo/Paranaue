use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

const HISTORY_FILE: &str = "history";
const FORGE_DIR: &str = "forge";
const DISABLE_HISTORY_ENV: &str = "FORGE_NO_HISTORY";

pub fn path() -> Option<PathBuf> {
    #[cfg(windows)]
    let base = env::var_os("APPDATA").map(PathBuf::from);

    #[cfg(target_os = "macos")]
    let base = env::var_os("HOME")
        .map(|home| PathBuf::from(home).join("Library").join("Application Support"));

    #[cfg(all(unix, not(target_os = "macos")))]
    let base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local").join("state")));

    base.map(|base| base.join(FORGE_DIR).join(HISTORY_FILE))
}

pub fn enabled() -> bool {
    !matches!(
        env::var(DISABLE_HISTORY_ENV).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

pub fn load(editor: &mut rustyline::DefaultEditor) -> io::Result<()> {
    if !enabled() {
        return Ok(());
    }

    let Some(path) = path() else { return Ok(()); };
    if path.is_file() {
        editor
            .load_history(&path)
            .map_err(|error| io::Error::other(format!("failed to load history: {error}")))?;
    }
    Ok(())
}

pub fn save(editor: &mut rustyline::DefaultEditor) -> io::Result<()> {
    if !enabled() {
        return Ok(());
    }

    let Some(path) = path() else { return Ok(()); };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    editor
        .save_history(&path)
        .map_err(|error| io::Error::other(format!("failed to save history: {error}")))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}
