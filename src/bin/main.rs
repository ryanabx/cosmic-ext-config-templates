use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use cosmic_ext_config_templates::{generic::GenericSchema, panel::PanelSchema, Schema};

// TEMPLATES
const UBUNTU_PANEL: &str = include_str!("../../base_templates/ubuntu_panel.ron");
const DEFAULT_PANEL: &str = include_str!("../../base_templates/default_panel.ron");
const WINDOWS_PANEL: &str = include_str!("../../base_templates/windows_panel.ron");
const WINDOWS10_PANEL: &str = include_str!("../../base_templates/windows10_panel.ron");
//

#[derive(Clone, Debug, ValueEnum)]
pub enum BaseTemplates {
    UbuntuPanel,
    DefaultPanel,
    WindowsPanel,
    Windows10Panel,
}

impl BaseTemplates {
    pub const fn to_string(&self) -> &'static str {
        match self {
            BaseTemplates::UbuntuPanel => UBUNTU_PANEL,
            BaseTemplates::DefaultPanel => DEFAULT_PANEL,
            BaseTemplates::WindowsPanel => WINDOWS_PANEL,
            BaseTemplates::Windows10Panel => WINDOWS10_PANEL,
        }
    }
}

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
        /// output file to create (by default ./template.ron)
        #[arg(short)]
        output: Option<PathBuf>,
        #[command(subcommand)]
        template_type: TemplateType,
    },
    /// Load a template from a file
    LoadFile {
        /// .ron template file to load
        from: PathBuf,
    },
    /// Load one of the premade templates
    LoadTemplate { template: BaseTemplates },
}

#[derive(Clone, Debug, Subcommand)]
enum TemplateType {
    /// generic template
    Generic {
        /// directories to include (for example, com.system76.CosmicPanel)
        include: Vec<PathBuf>,
    },
    /// panel template
    Panel,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();
    match args.action {
        Commands::Generate {
            output,
            template_type,
        } => {
            let output = output.unwrap_or(std::env::current_dir().unwrap().join("template.ron"));
            match template_type {
                TemplateType::Generic { include } => Schema::Generic(GenericSchema::generate(
                    include.iter().map(|s| s.as_path()).collect::<Vec<_>>(),
                    &output,
                )?),
                TemplateType::Panel => Schema::Panel(PanelSchema::generate()?),
            }
            .save(&output)?;
        }
        Commands::LoadFile { from } => {
            cosmic_ext_config_templates::load_template(Schema::from_file(&from)?)?
        }
        Commands::LoadTemplate { template } => {
            cosmic_ext_config_templates::load_template(schema_from_template(template)?)?
        }
    }
    Ok(())
}

fn schema_from_template(template: BaseTemplates) -> anyhow::Result<Schema> {
    Ok(ron::from_str::<Schema>(&template.to_string())?)
}
