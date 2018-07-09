use std::path::Path;
use std::process::Command;
use std::io::Result as IOResult;

use bytes::Bytes;
use grpc::Metadata;
use grpc::MetadataKey;

/// Represents the bytes of the macaroon
pub struct MacaroonData {
    raw: Vec<u8>,
}

impl MacaroonData {
    /// Reads the macaroon data from a file at the path
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        use std::io::{Error, ErrorKind};

        let output = Command::new("xxd")
            .args(&["-ps", "-u", "-c", "1000"])
            .arg(path.as_ref())
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(Error::new(ErrorKind::InvalidInput, error.as_ref()))
        } else {
            let mut data = output.stdout;
            data.retain(|&z|
                ((z >= '0' as _) && (z <= '9' as _)) |
                    ((z >= 'A' as _) && (z <= 'F' as _))
            );
            Ok(MacaroonData {
                raw: data,
            })
        }
    }

    /// Creates the `grpc::Metadata` instance that contain the provided macaroon
    pub fn metadata(&self) -> Metadata {
        let macaroon = Bytes::from(self.raw.clone());
        let mut metadata = Metadata::new();
        metadata.add(MetadataKey::from("macaroon"), macaroon);
        metadata
    }
}
