use std::{
    env, fs,
    path::{Path, PathBuf},
};

use clap::ValueEnum;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

// TEMPLATES
const UBUNTU_PANEL: &str = include_str!("../base_templates/ubuntu_panel.ron");
const DEFAULT_PANEL: &str = include_str!("../base_templates/default_panel.ron");
//

#[derive(Clone, Debug, ValueEnum)]
pub enum BaseTemplates {
    UbuntuPanel,
    DefaultPanel,
}

impl BaseTemplates {
    pub const fn to_string(&self) -> &'static str {
        match self {
            BaseTemplates::UbuntuPanel => UBUNTU_PANEL,
            BaseTemplates::DefaultPanel => DEFAULT_PANEL,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    overrides: Vec<Override>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            overrides: Vec::new(),
        }
    }

    pub fn from_file(f: &Path) -> anyhow::Result<Self> {
        log::info!("Loading template from {}", f.display());
        Ok(ron::from_str::<Schema>(&fs::read_to_string(f)?)?)
    }

    pub fn from_template(template: BaseTemplates) -> anyhow::Result<Self> {
        Ok(ron::from_str::<Schema>(&template.to_string())?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Override {
    path: PathBuf,
    setting: String,
}

pub fn load_template(schema: Schema) -> anyhow::Result<()> {
    let cosmic_conf_dir = Path::new(&env::var("HOME").unwrap()).join(".config/cosmic");
    for Override { path, setting } in schema.overrides {
        let path = cosmic_conf_dir.join(&path);
        log::debug!(
            "Saving to path {}\nschema: {}\n\n",
            &path.display(),
            &setting,
        );
        let _ = fs::create_dir_all(&path.parent().unwrap());
        fs::write(path, &setting)?;
    }
    Ok(())
}

pub fn generate_template(include_dirs: Vec<&Path>, outfile: &Path) -> anyhow::Result<()> {
    log::info!(
        "Generating template using includes: [{}] and saving to {}",
        include_dirs
            .iter()
            .map(|s| s.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join(", "),
        outfile.display()
    );
    let mut schema = Schema::new();
    let cosmic_conf_dir = Path::new(&env::var("HOME").unwrap()).join(".config/cosmic");
    for dir in include_dirs {
        let full_dir = cosmic_conf_dir.join(dir);
        for entry in WalkDir::new(&full_dir) {
            let entry = entry?;
            if entry.path().is_file() {
                schema.overrides.push(Override {
                    path: entry.path().strip_prefix(&cosmic_conf_dir)?.to_path_buf(),
                    setting: fs::read_to_string(entry.path())?,
                })
            }
        }
    }
    fs::write(
        outfile,
        ron::ser::to_string_pretty(&schema, PrettyConfig::default())?,
    )?;
    Ok(())
}
