mod cli;
mod complexity;
mod git_analyzer;
mod output;
mod score;
mod types;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Cli::parse();

    let git_metrics = git_analyzer::analyze_repository(
        &args.repo,
        args.since.as_deref(),
        args.until.as_deref(),
    )?;

    let files: Vec<PathBuf> = git_metrics.churn.keys().cloned().collect();

    let files_to_analyze: Vec<PathBuf> = files
        .iter()
        .filter(|path| {
            let path_str = path.to_string_lossy();

            if !args.include.is_empty() {
                if !args.include.iter().any(|pattern| path_str.contains(pattern)) {
                    return false;
                }
            }

            if !args.exclude.is_empty() {
                if args.exclude.iter().any(|pattern| path_str.contains(pattern)) {
                    return false;
                }
            }

            true
        })
        .map(|p| args.repo.join(p))
        .collect();

    let complexity_map = complexity::analyze_multiple_files(&files_to_analyze, &args.repo);

    let mut all_metrics = score::calculate_scores(&git_metrics, &complexity_map);

    all_metrics.truncate(args.top);

    if args.json {
        if let Some(path) = &args.out {
            output::save_json(&all_metrics, path)?;
            println!("JSON salvo em: {}", path.display());
        } else {
            let json = serde_json::to_string_pretty(&all_metrics)?;
            println!("{}", json);
        }
    } else if args.csv {
        if let Some(path) = &args.out {
            output::save_csv(&all_metrics, path)?;
            println!("CSV salvo em: {}", path.display());
        } else {
            println!("Use --out para especificar o arquivo de saída CSV");
        }
    } else if let Some(path) = &args.out {
        output::save_markdown(&all_metrics, path)?;
        println!("Markdown salvo em: {}", path.display());
    } else {
        output::print_table(&all_metrics);
    }

    Ok(())
}
