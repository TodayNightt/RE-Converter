use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct FileExt {
    parent: PathBuf,
    file_name: OsString,
    extension: OsString,
}

impl FileExt {
    pub fn path_without_extension(&self) -> PathBuf {
        self.parent.clone().join(self.file_name.clone())
    }

    pub fn path_with_extension(&self) -> PathBuf {
        self.path_without_extension()
            .with_extension(self.extension.clone())
    }

    pub fn file_name(&self) -> OsString {
        self.file_name.clone()
    }

    pub fn extension(&self) -> OsString {
        self.extension.clone()
    }
}

impl From<PathBuf> for FileExt {
    fn from(value: PathBuf) -> Self {
        let parent = value
            .parent()
            .unwrap_or_else(|| Path::new("")) // Default to empty path if parent is None
            .to_path_buf();

        let file_name = value
            .file_name()
            .unwrap_or_else(|| OsStr::new("")) // Default to empty OsString if file_name is None
            .to_os_string();

        let extension = value
            .extension()
            .unwrap_or_else(|| OsStr::new("")) // Default to empty OsString if extension is None
            .to_os_string();

        FileExt {
            parent,
            file_name,
            extension,
        }
    }
}
