use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, WriteMode};
use snafu::{Snafu};

use thruster::errors::ThrusterError;
use thruster::{async_middleware, middleware_fn};
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareResult};

mod routes;
use routes::controllers::plaintext;
use routes::controllers::four_oh_four;

#[derive(Debug, Snafu)]
enum Error {
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



#[middleware_fn]
async fn json_error_handler(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let res = next(context).await;

    // If there is not an error, return
    let mut err = match res {
        Ok(_) => return res,
        Err(err) => err,
    };

    // Generic handler, if we fail to downcast
    let e: Box<Error> = match err.cause.take().map(|cause| cause.downcast()) {
        Some(Ok(e)) => e,
        _ => {
            let mut context = err.context;

            context.body(&format!(
                "{{\"message\": \"{}\",\"success\":false}}",
                err.message
            ));
            context.status(err.status);

            return Ok(context);
        }
    };

    // Handle the Error variants
    let mut context = err.context;
    let status = match *e {
        Error::InvalidId { .. } => 400,
        Error::FileNotFound { .. } => 404,
    };

    log::info!("value received: {}", e);
    log::trace!("Trace is filtered");

    context.status(status);
    context.body(&format!("{{\"message\": \"{}\",\"success\":false}}", e));

    Ok(context)
}

fn main() {
    let logger = Logger::try_with_str("info") // Write all error, warn, and info messages
        .unwrap()
        .log_to_file(FileSpec::default())
        .rotate(
            // If the program runs long enough,
            Criterion::Age(Age::Day), // - create a new file every day
            Naming::Timestamps,       // - let the rotated files have a timestamp in their name
            Cleanup::KeepLogFiles(7), // - keep at most 7 log files
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .expect("Failed to start logger");
    println!("Starting server...");

    let mut app = App::<Request, Ctx, ()>::new_basic()
        .use_middleware("/", async_middleware!(Ctx, [json_error_handler]))
        .get("/hello", async_middleware!(Ctx, [plaintext]))
        .set404(async_middleware!(Ctx, [four_oh_four]));

    app.connection_timeout = 5000;

    let server = Server::new(app);
    server.start("0.0.0.0", 4321);
    logger.flush();
}
