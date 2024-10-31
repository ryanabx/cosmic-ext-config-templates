use std::{fs, path::Path};

use generic::GenericSchema;
use panel::PanelSchema;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

pub mod generic;
pub mod panel;
pub mod imports;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Schema {
    Generic(GenericSchema),
    Panel(PanelSchema),
}

impl Schema {
    pub fn from_file(f: &Path) -> anyhow::Result<Self> {
        log::info!("Loading template from {}", f.display());
        Ok(ron::from_str::<Schema>(&fs::read_to_string(f)?)?)
    }

    pub fn save(&self, outfile: &Path) -> anyhow::Result<()> {
        fs::write(
            outfile,
            ron::ser::to_string_pretty(&self, PrettyConfig::default())?,
        )?;
        Ok(())
    }
}

pub fn load_template(schema: Schema) -> anyhow::Result<()> {
    match schema {
        Schema::Generic(generic) => generic.load(),
        Schema::Panel(panel_schema) => panel_schema.load(),
    }
}
