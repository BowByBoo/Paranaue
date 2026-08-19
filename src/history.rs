use std::env;
use std::fs;
use std::io;

use crate::paths;

const DISABLE_HISTORY_ENV: &str = "FORGE_NO_HISTORY";

pub fn path() -> Option<std::path::PathBuf> {
    paths::history_file()
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
