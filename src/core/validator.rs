use snafu::Snafu;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::errors::ThrusterError;

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

trait ErrorExt {
    fn context(self, context: Ctx) -> ThrusterError<Ctx>;
}

impl<E: Into<Error>> ErrorExt for E {
    fn context(self, context: Ctx) -> ThrusterError<Ctx> {
        ThrusterError {
            context,
            message: "Failed to handle error".to_string(),
            status: 500,
            cause: Some(Box::new(self.into())),
        }
    }
}
