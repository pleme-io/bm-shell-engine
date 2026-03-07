mod config;
mod generator;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bm-shell-engine",
    about = "Blackmatter shell configuration engine — generates zsh config from YAML",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate all shell configuration files from a YAML config
    Generate {
        /// Path to shell.yaml config file
        #[arg(short, long, default_value = "shell.yaml")]
        config: PathBuf,
        /// Output directory for generated files
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// Validate a shell.yaml config file without generating
    Validate {
        /// Path to shell.yaml config file
        #[arg(short, long, default_value = "shell.yaml")]
        config: PathBuf,
    },
    /// Print the default configuration as YAML
    Defaults,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Generate { config, output } => {
            let cfg = config::load(&config).context("failed to load config")?;
            std::fs::create_dir_all(&output).context("failed to create output directory")?;
            generator::generate_all(&cfg, &output)?;
            println!("generated shell config in {}", output.display());
            Ok(())
        }
        Command::Validate { config } => {
            let cfg = config::load(&config).context("failed to load config")?;
            println!("config is valid");
            println!("  theme: {}", cfg.theme.name);
            println!("  plugins: {}", cfg.plugins.len());
            println!("  alias groups: {}", cfg.aliases.len());
            println!("  keybinding modes: {}", cfg.keybindings.len());
            Ok(())
        }
        Command::Defaults => {
            let cfg = config::ShellConfig::default();
            let yaml = serde_yaml::to_string(&cfg)?;
            print!("{yaml}");
            Ok(())
        }
    }
}
