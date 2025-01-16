use std::path::PathBuf;

use lib_utils::file::FileExt;

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    folder_title: String,
    xml_files: Vec<PathBuf>,
    video_files: Vec<FileExt>,
}

impl Bucket {
    pub fn title(&self) -> String {
        self.folder_title.clone()
    }

    pub fn xml_files(&self) -> Vec<PathBuf> {
        self.xml_files.clone()
    }

    pub fn video_files(&self) -> Vec<FileExt> {
        self.video_files.clone()
    }

    pub fn new(title: String) -> Self {
        Self {
            folder_title: title,
            ..Default::default()
        }
    }

    pub fn add_xml(&mut self, file: PathBuf) {
        self.xml_files.push(file);
    }

    pub fn add_video(&mut self, file: FileExt) {
        self.video_files.push(file);
    }
}
