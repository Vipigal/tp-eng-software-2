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
