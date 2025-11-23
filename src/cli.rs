use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "hotspot-analyzer")]
#[command(about = "Analisa hotspots de manutenção em repositórios Git", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,

    #[arg(long)]
    pub since: Option<String>,

    #[arg(long)]
    pub until: Option<String>,

    #[arg(long, value_delimiter = ',')]
    pub include: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    pub exclude: Vec<String>,

    #[arg(short, long, default_value_t = 10)]
    pub top: usize,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub csv: bool,

    #[arg(long)]
    pub out: Option<PathBuf>,
}
