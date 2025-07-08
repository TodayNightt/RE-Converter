use std::{collections::HashMap, ffi::OsStr, fs, fs::Metadata, path::PathBuf};

use chrono::Local;

use super::{bucket::Bucket, Result};

pub type Buckets = HashMap<String, Bucket>;

pub struct Sinker;

impl Sinker {
    pub fn sink(files: Vec<PathBuf>, need_sorting: bool) -> Result<Buckets> {
        let mut map = HashMap::with_capacity(30);
        let current_time = Local::now();
        for file in files.into_iter() {
            let title = if need_sorting {
                let metadata = Self::get_metadata(&file)?;

                let chrono_time: chrono::DateTime<Local> = metadata.modified()?.into();

                let datetime: lib_utils::time::Datetime = chrono_time.into();

                datetime.need_session().to_string()
            } else {
                let datetime: lib_utils::time::Datetime = current_time.into();
                datetime.to_string()
            };

            let b = map
                .entry(title.clone())
                .or_insert(Bucket::new(title.to_owned()));

            let file_type = file
                .extension()
                .unwrap_or(OsStr::new("unknown"))
                .to_str()
                // .ok_or_else(|| Error::ToOsStringError)?; This is to future me for error handling
                .unwrap()
                .to_lowercase();

            match file_type.as_str() {
                "xml" => b.add_xml(file.to_owned()),
                "mp4" => b.add_video(file.into()),
                _ => {}
            }
        }
        Ok(map)
    }

    fn get_metadata(file: &PathBuf) -> Result<Metadata> {
        let file = fs::File::open(file)?;

        file.metadata().map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr};

    use super::Sinker;

    #[test]
    fn get_metadata_test() {
        let m = Sinker::get_metadata(&PathBuf::from_str("./a.xml").unwrap()).unwrap();

        assert!(m.is_file())
    }
    #[test]
    fn sink_test() {
        let buckets = Sinker::sink(
            vec![
                PathBuf::from_str("a.xml").unwrap(),
                PathBuf::from_str("test.mp4").unwrap(),
            ],
            true,
        )
        .unwrap();

        assert!(buckets.contains_key("241106B"));

        let the_bucket = buckets.get("241106B").unwrap();

        assert_eq!(the_bucket.title(), "241106B");

        let xml = the_bucket.xml_files();

        assert_eq!(xml.len(), 1);

        assert_eq!(xml.first().unwrap(), &PathBuf::from_str("a.xml").unwrap());

        let v = the_bucket.video_files();

        assert_eq!(v.len(), 1);

        // assert_eq!(v.first().unwrap(), &PathBuf::from_str("test.mp4").unwrap())
    }
}
