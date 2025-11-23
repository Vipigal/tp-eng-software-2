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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_calculate_scores_basic() {
        let mut git_metrics = GitMetrics {
            churn: HashMap::new(),
            authors: HashMap::new(),
        };

        let path = PathBuf::from("test.rs");
        git_metrics.churn.insert(path.clone(), 100);

        let mut authors = HashSet::new();
        authors.insert("Alice".to_string());
        git_metrics.authors.insert(path.clone(), authors);

        let mut complexity_map = HashMap::new();
        complexity_map.insert(path.clone(), 50.0);

        let results = calculate_scores(&git_metrics, &complexity_map);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].churn, 100);
        assert_eq!(results[0].complexity, 50.0);
        assert_eq!(results[0].authors, 1);
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_calculate_scores_sorted_by_score() {
        let mut git_metrics = GitMetrics {
            churn: HashMap::new(),
            authors: HashMap::new(),
        };

        let path1 = PathBuf::from("low_score.rs");
        let path2 = PathBuf::from("high_score.rs");

        git_metrics.churn.insert(path1.clone(), 10);
        git_metrics.churn.insert(path2.clone(), 100);

        let mut authors1 = HashSet::new();
        authors1.insert("Alice".to_string());
        git_metrics.authors.insert(path1.clone(), authors1);

        let mut authors2 = HashSet::new();
        authors2.insert("Bob".to_string());
        git_metrics.authors.insert(path2.clone(), authors2);

        let mut complexity_map = HashMap::new();
        complexity_map.insert(path1.clone(), 10.0);
        complexity_map.insert(path2.clone(), 100.0);

        let results = calculate_scores(&git_metrics, &complexity_map);

        // Deve estar ordenado por score decrescente
        assert!(results[0].score > results[1].score);
        assert_eq!(results[0].path, path2);
    }

    #[test]
    fn test_calculate_scores_multiple_authors_lower_score() {
        let mut git_metrics = GitMetrics {
            churn: HashMap::new(),
            authors: HashMap::new(),
        };

        let path1 = PathBuf::from("single_author.rs");
        let path2 = PathBuf::from("multiple_authors.rs");

        // Mesmo churn para ambos
        git_metrics.churn.insert(path1.clone(), 100);
        git_metrics.churn.insert(path2.clone(), 100);

        let mut authors1 = HashSet::new();
        authors1.insert("Alice".to_string());
        git_metrics.authors.insert(path1.clone(), authors1);

        let mut authors2 = HashSet::new();
        authors2.insert("Bob".to_string());
        authors2.insert("Charlie".to_string());
        authors2.insert("Dave".to_string());
        git_metrics.authors.insert(path2.clone(), authors2);

        let mut complexity_map = HashMap::new();
        // Mesma complexidade para ambos
        complexity_map.insert(path1.clone(), 50.0);
        complexity_map.insert(path2.clone(), 50.0);

        let results = calculate_scores(&git_metrics, &complexity_map);

        // Arquivo com um único autor deve ter score maior (mais penalidade)
        let score1 = results.iter().find(|m| m.path == path1).unwrap().score;
        let score2 = results.iter().find(|m| m.path == path2).unwrap().score;

        assert!(score1 > score2);
    }

    #[test]
    fn test_calculate_scores_zero_complexity() {
        let mut git_metrics = GitMetrics {
            churn: HashMap::new(),
            authors: HashMap::new(),
        };

        let path = PathBuf::from("test.rs");
        git_metrics.churn.insert(path.clone(), 100);

        let mut authors = HashSet::new();
        authors.insert("Alice".to_string());
        git_metrics.authors.insert(path.clone(), authors);

        let complexity_map = HashMap::new(); // Sem complexidade

        let results = calculate_scores(&git_metrics, &complexity_map);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].complexity, 0.0);
        assert_eq!(results[0].score, 0.0); // Score deve ser 0 se complexidade é 0
    }
}
