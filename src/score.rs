use crate::git_analyzer::GitMetrics;
use crate::types::FileMetrics;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn calculate_scores(
    git_metrics: &GitMetrics,
    complexity_map: &HashMap<PathBuf, f64>,
) -> Vec<FileMetrics> {
    let mut file_metrics = Vec::new();

    let max_churn = git_metrics.churn.values().max().copied().unwrap_or(1) as f64;
    let max_complexity = complexity_map.values().copied().fold(0.0f64, f64::max).max(1.0);

    for (path, &churn) in &git_metrics.churn {
        let complexity = *complexity_map.get(path).unwrap_or(&0.0);
        let authors = git_metrics
            .authors
            .get(path)
            .map(|s| s.len())
            .unwrap_or(1);

        let churn_norm = churn as f64 / max_churn;
        let complex_norm = complexity / max_complexity;
        let authorship_penalty = 1.0 / (1.0 + (authors as f64).ln_1p());

        let score = 100.0 * churn_norm * complex_norm * authorship_penalty;

        file_metrics.push(FileMetrics {
            path: path.clone(),
            churn,
            complexity,
            authors,
            score,
        });
    }

    file_metrics.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    file_metrics
}
