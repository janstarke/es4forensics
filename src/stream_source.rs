use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;

pub(crate) enum StreamSource {
    Stdin,
    File(Box<dyn BufRead + Send>),
}

impl StreamSource {
    pub fn from(filename: &str) -> Result<Self> {
        if filename == "-" {
            Ok(StreamSource::Stdin)
        } else {
            let file = BufReader::new(File::open(filename)?);

            #[cfg(not(feature = "gzip"))]
            let reader: Box<dyn BufRead + Send> = Box::new(file);

            #[cfg(feature = "gzip")]
            let reader = Self::open_gzip(filename, file);

            Ok(StreamSource::File(reader))
        }
    }

    #[cfg(feature = "gzip")]
    fn open_gzip<R: Read + Send + 'static>(filename: &str, file: R) -> Box<dyn Read + Send> {
        if filename.ends_with(".gz") {
            Box::new(GzDecoder::new(file))
        } else {
            Box::new(file)
        }
    }
}

impl From<StreamSource> for Box<dyn BufRead + Send> {
    fn from(me: StreamSource) -> Self {
        match me {
            StreamSource::Stdin => Box::new(BufReader::new(std::io::stdin())),
            StreamSource::File(f) => f,
        }
    }
}
