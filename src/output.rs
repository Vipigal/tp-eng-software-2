use crate::types::FileMetrics;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct TableRow {
    #[tabled(rename = "Arquivo")]
    path: String,
    #[tabled(rename = "Churn")]
    churn: usize,
    #[tabled(rename = "Complexidade")]
    complexity: String,
    #[tabled(rename = "Autores")]
    authors: usize,
    #[tabled(rename = "Score")]
    score: String,
}

impl From<&FileMetrics> for TableRow {
    fn from(m: &FileMetrics) -> Self {
        Self {
            path: m.path.display().to_string(),
            churn: m.churn,
            complexity: format!("{:.2}", m.complexity),
            authors: m.authors,
            score: format!("{:.2}", m.score),
        }
    }
}

pub fn print_table(metrics: &[FileMetrics]) {
    let rows: Vec<TableRow> = metrics.iter().map(TableRow::from).collect();
    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);
}

pub fn save_json(metrics: &[FileMetrics], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(metrics)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn save_csv(metrics: &[FileMetrics], path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(&["Arquivo", "Churn", "Complexidade", "Autores", "Score"])?;

    for m in metrics {
        writer.write_record(&[
            m.path.display().to_string(),
            m.churn.to_string(),
            format!("{:.2}", m.complexity),
            m.authors.to_string(),
            format!("{:.2}", m.score),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn save_markdown(metrics: &[FileMetrics], path: &Path) -> Result<()> {
    let mut content = String::from("# Análise de Hotspots\n\n");
    content.push_str("| Arquivo | Churn | Complexidade | Autores | Score |\n");
    content.push_str("|---------|-------|--------------|---------|-------|\n");

    for m in metrics {
        content.push_str(&format!(
            "| {} | {} | {:.2} | {} | {:.2} |\n",
            m.path.display(),
            m.churn,
            m.complexity,
            m.authors,
            m.score
        ));
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_metrics() -> Vec<FileMetrics> {
        vec![
            FileMetrics {
                path: PathBuf::from("src/main.rs"),
                churn: 100,
                complexity: 50.5,
                authors: 3,
                score: 75.25,
            },
            FileMetrics {
                path: PathBuf::from("src/lib.rs"),
                churn: 50,
                complexity: 25.3,
                authors: 2,
                score: 40.15,
            },
        ]
    }

    #[test]
    fn test_table_row_from_file_metrics() {
        let metrics = FileMetrics {
            path: PathBuf::from("test.rs"),
            churn: 100,
            complexity: 50.5,
            authors: 2,
            score: 75.0,
        };

        let row = TableRow::from(&metrics);
        assert_eq!(row.path, "test.rs");
        assert_eq!(row.churn, 100);
        assert_eq!(row.complexity, "50.50");
        assert_eq!(row.authors, 2);
        assert_eq!(row.score, "75.00");
    }

    #[test]
    fn test_save_json() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("output.json");

        let metrics = create_test_metrics();
        let result = save_json(&metrics, &json_path);

        assert!(result.is_ok());
        assert!(json_path.exists());

        let content = std::fs::read_to_string(&json_path).unwrap();
        assert!(content.contains("main.rs"));
        assert!(content.contains("lib.rs"));
    }

    #[test]
    fn test_save_csv() {
        let temp_dir = TempDir::new().unwrap();
        let csv_path = temp_dir.path().join("output.csv");

        let metrics = create_test_metrics();
        let result = save_csv(&metrics, &csv_path);

        assert!(result.is_ok());
        assert!(csv_path.exists());

        let content = std::fs::read_to_string(&csv_path).unwrap();
        assert!(content.contains("Arquivo"));
        assert!(content.contains("main.rs"));
        assert!(content.contains("lib.rs"));
    }

    #[test]
    fn test_save_markdown() {
        let temp_dir = TempDir::new().unwrap();
        let md_path = temp_dir.path().join("output.md");

        let metrics = create_test_metrics();
        let result = save_markdown(&metrics, &md_path);

        assert!(result.is_ok());
        assert!(md_path.exists());

        let content = std::fs::read_to_string(&md_path).unwrap();
        assert!(content.contains("# Análise de Hotspots"));
        assert!(content.contains("main.rs"));
        assert!(content.contains("lib.rs"));
        assert!(content.contains("|")); // Markdown table syntax
    }
}
