use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not parse id: {}", id))]
    InvalidId {
        id: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Could not open config at {}: {}", filename.display(), source))]
    FileNotFound {
        filename: std::path::PathBuf,
        source: std::io::Error,
    },
}
