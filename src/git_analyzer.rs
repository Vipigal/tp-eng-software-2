use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use git2::Repository;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct GitMetrics {
    pub churn: HashMap<PathBuf, usize>,
    pub authors: HashMap<PathBuf, HashSet<String>>,
}

impl GitMetrics {
    pub fn new() -> Self {
        Self {
            churn: HashMap::new(),
            authors: HashMap::new(),
        }
    }

    pub fn add_change(&mut self, path: PathBuf, lines_changed: usize, author: String) {
        *self.churn.entry(path.clone()).or_insert(0) += lines_changed;
        self.authors
            .entry(path)
            .or_insert_with(HashSet::new)
            .insert(author);
    }
}

pub fn analyze_repository(
    repo_path: &PathBuf,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<GitMetrics> {
    let repo = Repository::open(repo_path).context("Falha ao abrir repositório")?;

    let since_time = parse_date(since)?;
    let until_time = parse_date(until)?;

    let mut metrics = GitMetrics::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let commit_time = commit.time();
        let commit_datetime = Utc.timestamp_opt(commit_time.seconds(), 0).unwrap();

        if let Some(since) = since_time {
            if commit_datetime < since {
                continue;
            }
        }

        if let Some(until) = until_time {
            if commit_datetime > until {
                continue;
            }
        }

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();

        let tree = commit.tree()?;
        let parent_count = commit.parent_count();

        if parent_count == 0 {
            continue;
        }

        for i in 0..parent_count {
            let parent = commit.parent(i)?;
            let parent_tree = parent.tree()?;

            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

            // Obter estatísticas do diff
            let stats = diff.stats()?;
            let files_changed = stats.files_changed();

            // Para cada arquivo, somar adições e deleções
            for file_idx in 0..files_changed {
                if let Some(delta) = diff.get_delta(file_idx) {
                    if let Some(path) = delta.new_file().path() {
                        // Obter o patch para contar linhas
                        if let Ok(Some(patch)) = git2::Patch::from_diff(&diff, file_idx) {
                            if let Ok((_, additions, deletions)) = patch.line_stats() {
                                let churn = additions + deletions;

                                if churn > 0 {
                                    let path_buf = PathBuf::from(path);
                                    metrics.add_change(path_buf, churn, author_name.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(metrics)
}

fn parse_date(date_str: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    match date_str {
        None => Ok(None),
        Some(s) => {
            let parsed = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .context("Data inválida, use formato YYYY-MM-DD")?
                .and_hms_opt(0, 0, 0)
                .unwrap();
            Ok(Some(DateTime::from_naive_utc_and_offset(parsed, Utc)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_metrics_new() {
        let metrics = GitMetrics::new();
        assert!(metrics.churn.is_empty());
        assert!(metrics.authors.is_empty());
    }

    #[test]
    fn test_git_metrics_add_change() {
        let mut metrics = GitMetrics::new();
        let path = PathBuf::from("src/main.rs");
        metrics.add_change(path.clone(), 10, "Alice".to_string());

        assert_eq!(*metrics.churn.get(&path).unwrap(), 10);
        assert_eq!(metrics.authors.get(&path).unwrap().len(), 1);
    }

    #[test]
    fn test_git_metrics_add_multiple_changes_same_file() {
        let mut metrics = GitMetrics::new();
        let path = PathBuf::from("src/main.rs");

        metrics.add_change(path.clone(), 10, "Alice".to_string());
        metrics.add_change(path.clone(), 5, "Bob".to_string());

        assert_eq!(*metrics.churn.get(&path).unwrap(), 15);
        assert_eq!(metrics.authors.get(&path).unwrap().len(), 2);
    }

    #[test]
    fn test_git_metrics_multiple_authors_same_file() {
        let mut metrics = GitMetrics::new();
        let path = PathBuf::from("src/main.rs");

        metrics.add_change(path.clone(), 5, "Alice".to_string());
        metrics.add_change(path.clone(), 3, "Bob".to_string());
        metrics.add_change(path.clone(), 2, "Charlie".to_string());

        assert_eq!(metrics.authors.get(&path).unwrap().len(), 3);
    }

    #[test]
    fn test_parse_date_none() {
        let result = parse_date(None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_date_valid() {
        let result = parse_date(Some("2024-01-15")).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_date_invalid() {
        let result = parse_date(Some("invalid-date"));
        assert!(result.is_err());
    }
}
