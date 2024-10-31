use crate::imports::cosmic_panel_button_config::CosmicPanelButtonConfig;
use cosmic_config::CosmicConfigEntry;
use cosmic_panel_config::CosmicPanelContainerConfig;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PanelSchema {
    panel_config: CosmicPanelContainerConfig,
    panel_config_version: u64,
    panel_button_config: CosmicPanelButtonConfig,
    panel_button_config_version: u64,
}

pub const PANEL_BUTTON_CONFIG_VERSION: u64 = 1;
pub const PANEL_CONFIG_VERSION: u64 = 1;

fn get_current_panel_config(
) -> anyhow::Result<(CosmicPanelContainerConfig, CosmicPanelButtonConfig)> {
    Ok((
        CosmicPanelContainerConfig::load().unwrap_or_else(|_| {
            log::error!("Could not load cosmic panel container config!");
            CosmicPanelContainerConfig::default()
        }),
        CosmicPanelButtonConfig::get_entry(&cosmic_config::Config::new(
            "com.system76.CosmicPanelButton",
            PANEL_BUTTON_CONFIG_VERSION,
        )?)
        .unwrap_or_default(),
    ))
}

impl PanelSchema {
    pub fn load(&self) -> anyhow::Result<()> {
        self.panel_config.write_entries()?;
        self.panel_button_config
            .write_entry(&cosmic_config::Config::new(
                "com.system76.CosmicPanelButton",
                self.panel_button_config_version,
            )?)?;
        Ok(())
    }

    pub fn generate() -> anyhow::Result<Self> {
        let (panel_config, panel_button_config) = get_current_panel_config()?;
        Ok(Self {
            panel_config,
            panel_config_version: PANEL_CONFIG_VERSION,
            panel_button_config,
            panel_button_config_version: PANEL_BUTTON_CONFIG_VERSION,
        })
    }
}
