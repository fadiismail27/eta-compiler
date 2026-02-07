use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Reads the entire source file into a String
pub fn read_source_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Computes the output path for a .lexed file
/// - If output_dir is None: foo/bar.eta -> foo/bar.lexed
/// - If output_dir is Some("out"): foo/bar.eta -> out/foo/bar.lexed
pub fn compute_output_path(source_path: &str, output_dir: Option<&str>) -> PathBuf {
    let source = Path::new(source_path);
    let stem = source.file_stem().unwrap_or_default();
    let parent = source.parent().unwrap_or(Path::new(""));

    let lexed_name = format!("{}.lexed", stem.to_string_lossy());

    match output_dir {
        Some(dir) => {
            // Preserve directory structure under output dir
            // Strip leading / from parent if it's absolute
            let relative_parent = parent.strip_prefix("/").unwrap_or(parent);
            Path::new(dir).join(relative_parent).join(lexed_name)
        }
        None => {
            // Same directory as source
            parent.join(lexed_name)
        }
    }
}

/// Writes the .lexed content to the output file, creating directories as needed
pub fn write_lexed_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
