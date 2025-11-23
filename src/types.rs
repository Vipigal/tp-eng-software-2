use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub path: PathBuf,
    pub churn: usize,
    pub complexity: f64,
    pub authors: usize,
    pub score: f64,
}

