use std::io::Read as _;
use std::io::Write as _;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

trait DataSource {
    fn read(&self) -> String;
    fn write(&self, data: String);
}

struct FileSource {
    path: std::path::PathBuf,
}
impl DataSource for FileSource {
    fn read(&self) -> String {
        std::fs::read_to_string(&self.path).unwrap()
    }

    fn write(&self, data: String) {
        std::fs::write(&self.path, data).unwrap()
    }
}

struct DataSourceDecorator {
    wrapee: Box<dyn DataSource>,
}
impl DataSourceDecorator {
    fn new(wrapee: impl DataSource + 'static) -> Self {
        Self {
            wrapee: Box::new(wrapee),
        }
    }
}
impl DataSource for DataSourceDecorator {
    fn read(&self) -> String {
        self.wrapee.read()
    }
    fn write(&self, data: String) {
        self.wrapee.write(data)
    }
}

struct EncryptionDecorator {
    base: DataSourceDecorator,
}
impl DataSource for EncryptionDecorator {
    fn read(&self) -> String {
        let encrypted = self.base.read();
        String::from_utf8(STANDARD.decode(encrypted).unwrap()).unwrap()
    }

    fn write(&self, data: String) {
        let encrypted = STANDARD.encode(data);
        self.base.write(encrypted);
    }
}

struct CompressionDecorator {
    base: DataSourceDecorator,
}
impl DataSource for CompressionDecorator {
    fn read(&self) -> String {
        let b64 = self.base.read();
        let compressed = STANDARD.decode(b64).unwrap();
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut output = String::new();
        decoder.read_to_string(&mut output).unwrap();
        output
    }

    fn write(&self, data: String) {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data.as_bytes()).unwrap();
        let compressed = encoder.finish().unwrap();
        self.base.write(STANDARD.encode(compressed));
    }
}

fn main() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("decorator_demo.txt");

    // 1. plain file
    let source = FileSource { path: path.clone() };
    source.write(String::from("plain text"));
    println!("plain: {}", source.read());

    // 2. encryption only
    let encrypt_source = EncryptionDecorator {
        base: DataSourceDecorator::new(FileSource { path: path.clone() }),
    };
    encrypt_source.write(String::from("secret data"));
    println!("enc:   {}", encrypt_source.read());

    // 3. compression only
    let compress_source = CompressionDecorator {
        base: DataSourceDecorator::new(FileSource { path: path.clone() }),
    };
    let long = std::iter::repeat_n("hello ", 20).collect::<String>();
    compress_source.write(long.clone());
    println!("comp:  {}", compress_source.read());

    // 4. encryption + compression
    let both = CompressionDecorator {
        base: DataSourceDecorator::new(EncryptionDecorator {
            base: DataSourceDecorator::new(FileSource { path: path.clone() }),
        }),
    };
    both.write(String::from("decorators are composable!"));
    println!("both:  {}", both.read());
}
