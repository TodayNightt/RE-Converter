use lib_utils::file::FileExt;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct _Bucket {
    folder_title: String,
    xml_files: Vec<PathBuf>,
    video_files: Vec<FileExt>,
}

impl _Bucket {
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
#[derive(Debug, Default, Clone)]
pub struct Bucket {
    folder_title: Arc<str>,
    xml_files: Arc<[PathBuf]>,
    video_files: Arc<[FileExt]>,
}

impl Bucket {
    pub fn title(&self) -> Arc<str> {
        self.folder_title.clone()
    }

    pub fn xml_files(&self) -> Arc<[PathBuf]> {
        self.xml_files.clone()
    }

    pub fn video_files(&self) -> Arc<[FileExt]> {
        self.video_files.clone()
    }

    pub fn into_parts(self) -> (Arc<str>, Arc<[PathBuf]>, Arc<[FileExt]>) {
        (self.folder_title, self.xml_files, self.video_files)
    }
}

impl From<_Bucket> for Bucket {
    fn from(value: _Bucket) -> Self {
        Self {
            folder_title: Arc::from(value.folder_title),
            xml_files: Arc::from(value.xml_files),
            video_files: Arc::from(value.video_files),
        }
    }
}
