use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

pub fn load_cargo_toml_content<T>(path: &Path) -> Option<T>
where
    T: DeserializeOwned,
{
    // Attempt to read the file
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", path.display(), e);
            return None;
        }
    };

    // Attempt to parse the TOML content
    match toml::from_str(&content) {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            eprintln!("Failed to parse {}: {}", path.display(), e);
            None
        }
    }
}
