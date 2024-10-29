use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cosmic_ext_config_templates::{BaseTemplates, Schema};

/// cosmic-ext-templates
/// create cosmic config templates efficiently!
#[derive(Clone, Debug, Parser)]
#[command(about, version)]
struct Cli {
    /// action to take
    #[command(subcommand)]
    action: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Generate a template
    Generate {
        /// output file to create (for example, ~/template.ron)
        output: PathBuf,
        /// directories to include (for example, com.system76.CosmicPanel)
        include: Vec<PathBuf>,
    },
    /// Load a template from a file
    LoadFile {
        /// .ron template file to load
        from: PathBuf,
    },
    /// Load one of the premade templates
    LoadTemplate { template: BaseTemplates },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();
    match args.action {
        Commands::Generate { output, include } => cosmic_ext_config_templates::generate_template(
            include.iter().map(|s| s.as_path()).collect::<Vec<_>>(),
            &output,
        )?,
        Commands::LoadFile { from } => {
            cosmic_ext_config_templates::load_template(Schema::from_file(&from)?)?
        }
        Commands::LoadTemplate { template } => {
            cosmic_ext_config_templates::load_template(Schema::from_template(template)?)?
        }
    }
    Ok(())
}
