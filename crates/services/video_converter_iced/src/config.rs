pub use error::{Error, Result};
use lib_core::types::ConverterOptions;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use tokio::sync::RwLock;

mod error {
    use std::fmt::{Display, Formatter};

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        SerdeError(String),
        IOError(String),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::SerdeError(msg) => f.write_str(msg),
                Error::IOError(msg) => f.write_str(msg),
            }
        }
    }

    impl core::error::Error for Error {}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub last_saved: Option<ConverterOptions>,
    #[serde(skip)]
    pub saved_path: PathBuf,
}

impl Config {
    pub fn get_instance() -> &'static RwLock<Config> {
        static INSTANCE: OnceCell<RwLock<Config>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let mut app_dir = directories::ProjectDirs::from("com", "re-converter", "app")
                .unwrap()
                .config_local_dir()
                .to_path_buf();

            app_dir.push("config.json");
            let config = Config::load(&app_dir).unwrap();

            RwLock::new(config)
        })
    }

    pub fn load(path: &Path) -> Result<Self> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let config = serde_json::from_reader(reader)
                    .map_err(|e| Error::SerdeError(format!("Failed to deserialize JSON: {}", e)))?;

                Ok(Config {
                    saved_path: path.to_path_buf(),
                    ..config
                })
            }
            Err(_) => {
                let default_config = Config {
                    saved_path: path.to_path_buf(),
                    ..Self::default()
                };

                default_config
                    .save_config()
                    .map_err(|e| Error::IOError(format!("Failed to save default config: {}", e)))?;
                Ok(default_config)
            }
        }
    }

    fn save_config(&self) -> Result<()> {
        if let Some(parent_dir) = self.saved_path.parent() {
            // Create the parent directory if it doesn't exist.
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|e| Error::IOError(format!("Failed to create directory: {}", e)))?;
            }
        }

        let file = File::create(self.saved_path.as_path())
            .map_err(|e| Error::IOError(format!("Failed to create file: {}", e)))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| Error::SerdeError(format!("Failed to serialize JSON: {}", e)))
    }
    pub fn last_saved(&self) -> &Option<ConverterOptions> {
        &self.last_saved
    }

    pub fn update_last_saved_and_save(&mut self, options: ConverterOptions) -> Result<()> {
        if self.last_saved.as_ref().is_some_and(|ls| ls.eq(&options)) {
            return Ok(());
        }
        self.last_saved = Some(options);
        self.save_config()
    }
}
