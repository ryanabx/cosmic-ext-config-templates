use std::{
    env, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenericSchema {
    overrides: Vec<Override>,
}

impl GenericSchema {
    pub fn generate(include_dirs: Vec<&Path>, outfile: &Path) -> anyhow::Result<Self> {
        log::info!(
            "Generating template using includes: [{}] and saving to {}",
            include_dirs
                .iter()
                .map(|s| s.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join(", "),
            outfile.display()
        );
        let mut schema = GenericSchema {
            overrides: Vec::new(),
        };
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
        Ok(schema)
    }

    pub fn load(&self) -> anyhow::Result<()> {
        let cosmic_conf_dir = Path::new(&env::var("HOME").unwrap()).join(".config/cosmic");
        for Override { path, setting } in &self.overrides {
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Override {
    path: PathBuf,
    setting: String,
}
