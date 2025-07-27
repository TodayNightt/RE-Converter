use once_cell::sync::OnceCell;
use rust_embed::Embed;
use std::path::PathBuf;

pub mod svg;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
pub struct Assets;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../../../binaries"]
struct Binaries;

pub fn ffmpeg_instance() -> &'static PathBuf {
    static INSTANCE: OnceCell<PathBuf> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        use std::fs::File;
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let ffmpeg_path = temp_dir.join("ffmpeg.exe");

        if !ffmpeg_path.exists() {
            {
                let mut file = File::create(&ffmpeg_path).unwrap();
                file.write_all(&Binaries::get("ffmpeg").unwrap().data)
                    .unwrap();
            }
        }
        ffmpeg_path
    })
}
