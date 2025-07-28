#[cfg(feature = "embedded")]
use once_cell::sync::OnceCell;
#[cfg(feature = "embedded")]
use std::path::PathBuf;

use rust_embed::Embed;
pub mod svg;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
pub struct Assets;

#[cfg(feature = "embedded")]
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../../../binaries"]
struct Binaries;

#[cfg(feature = "embedded")]
pub fn ffmpeg_instance() -> &'static PathBuf {
    static INSTANCE: OnceCell<PathBuf> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        use std::fs::File;
        use std::io::Write;

        let ffmpeg_name = if cfg!(target_os = "windows") {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        };

        let temp_dir = std::env::temp_dir();

        let ffmpeg_path = temp_dir.join(ffmpeg_name);

        if !ffmpeg_path.exists() {
            {
                let mut file = File::create(&ffmpeg_path).unwrap();
                file.write_all(&Binaries::get(ffmpeg_name).unwrap().data)
                    .unwrap();
            }
        }
        ffmpeg_path
    })
}
