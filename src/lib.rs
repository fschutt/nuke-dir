use std::path::Path;

/// Deletes a directory by removing all files and subdirectories first,
/// then deleting the top-level directory
pub fn nuke_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
    use std::fs;
    let path = path.as_ref();
    for entry in
        fs::read_dir(path).map_err(|e| format!("{}: {e}", path.display()))?
    {
        let entry = entry.map_err(|e| format!("{}: {}", path.display(), e))?;
        let path = entry.path();

        let file_type = entry
            .file_type()
            .map_err(|e| format!("{}: {}", path.display(), e))?;

        if file_type.is_dir() {
            nuke_dir(&path)?;
            fs::remove_dir(&path)
                .map_err(|e| format!("{}: {}", path.display(), e))?;
        } else {
            fs::remove_file(&path)
                .map_err(|e| format!("{}: {}", path.display(), e))?;
        }
    }

    Ok(())
}
