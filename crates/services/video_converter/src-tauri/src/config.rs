// use std::{
//     fs::{self, File},
//     io::{BufReader, BufWriter},
//     path::{Path, PathBuf},
// };
//
// use crate::{Error, Result};
// use lib_core::ConverterOptions;
// use serde::{Deserialize, Serialize};
// use typeshare::typeshare;
// // use specta::Type;
//
// #[derive(Debug, Serialize, Deserialize, Default, Clone)]
// #[typeshare]
// pub struct Config {
//     pub last_saved: Option<ConverterOptions>,
//     #[serde(skip)]
//     pub saved_path: PathBuf,
// }
//
// impl Config {
//     pub fn load(path: &Path) -> Result<Config> {
//         println!("{:?}", path);
//         match File::open(path) {
//             Ok(file) => {
//                 let reader = BufReader::new(file);
//                 let config = serde_json::from_reader(reader)
//                     .map_err(|e| Error::SerdeError(format!("Failed to deserialize JSON: {}", e)))?;
//
//                 Ok(Config {
//                     saved_path: path.to_path_buf(),
//                     ..config
//                 })
//             }
//             Err(_) => {
//                 let default_config = Config {
//                     saved_path: path.to_path_buf(),
//                     ..Self::default()
//                 };
//
//                 default_config
//                     .save_config()
//                     .map_err(|e| Error::IoError(format!("Failed to save default config: {}", e)))?;
//                 Ok(default_config)
//             }
//         }
//     }
//
//     fn save_config(&self) -> Result<()> {
//         if let Some(parent) = self.saved_path.parent() {
//             // Ensure the parent directory exists
//             fs::create_dir_all(parent)
//                 .map_err(|e| Error::IoError(format!("Failed to create directory: {}", e)))?;
//         }
//
//         let file = File::create(self.saved_path.as_path())
//             .map_err(|e| Error::IoError(format!("Failed to create file: {}", e)))?;
//         let writer = BufWriter::new(file);
//         serde_json::to_writer_pretty(writer, self)
//             .map_err(|e| Error::SerdeError(format!("Failed to serialize JSON: {}", e)))
//     }
//     pub fn last_saved(&self) -> &Option<ConverterOptions> {
//         &self.last_saved
//     }
//
//     pub fn update_last_saved(&mut self, options: ConverterOptions) {
//         self.last_saved = Some(options)
//     }
//
//     pub fn update_last_saved_and_save(&mut self, options: ConverterOptions) -> Result<()> {
//         self.update_last_saved(options);
//         self.save_config()
//     }
// }
