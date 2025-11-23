use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokei::{Config, Languages};

pub fn analyze_file_complexity(path: &Path) -> Result<f64> {
    if !path.exists() || !path.is_file() {
        return Ok(0.0);
    }

    let mut languages = Languages::new();
    let config = Config::default();

    let paths = vec![path.to_path_buf()];
    languages.get_statistics(&paths, &[], &config);

    let mut total_code = 0;

    for (_, language) in languages {
        total_code += language.code;
    }

    Ok(total_code as f64)
}

pub fn analyze_multiple_files(
    paths: &[PathBuf],
    repo_path: &Path,
) -> HashMap<PathBuf, f64> {
    let mut complexity_map = HashMap::new();

    for path in paths {
        if path.exists() && path.is_file() {
            if let Ok(complexity) = analyze_file_complexity(path) {
                // Calcular caminho relativo ao repositório
                let relative_path = path
                    .strip_prefix(repo_path)
                    .unwrap_or(path);
                complexity_map.insert(relative_path.to_path_buf(), complexity);
            }
        }
    }

    complexity_map
}
