// ----- Facade ----
pub struct VideoConverter;
impl VideoConverter {
    fn convert(&self, fp: std::path::PathBuf, into_format: String) {
        println!("Reading file \"{}\"...", fp.display());
        println!("Extracting source codec...");
        println!("Destination format: {into_format}");
        println!("Reading bitrate...");
        println!("Converting bitrate...");
        println!("Fixing audio...");
        println!("Writing to \"{}.{into_format}\"...", fp.display());
        println!("Conversion done!");
    }
}

fn main() {
    VideoConverter {}.convert("foobar.mp4".into(), "mp3".into());
}
