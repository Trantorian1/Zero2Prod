#![cfg(feature = "fixtures")]

use futures::{FutureExt, future::BoxFuture};

pub struct TracingConsumer {
    level: tracing::Level,
}

impl Default for TracingConsumer {
    fn default() -> Self {
        Self { level: tracing::Level::INFO }
    }
}

impl TracingConsumer {
    pub fn new(level: tracing::Level) -> Self {
        Self { level }
    }

    fn log_bytes(&self, bytes: &[u8]) {
        let msg = String::from_utf8_lossy(bytes);
        match self.level {
            tracing::Level::TRACE => tracing::event!(tracing::Level::TRACE, "{}", &msg),
            tracing::Level::DEBUG => tracing::event!(tracing::Level::DEBUG, "{}", &msg),
            tracing::Level::INFO => tracing::event!(tracing::Level::INFO, "{}", &msg),
            tracing::Level::WARN => tracing::event!(tracing::Level::WARN, "{}", &msg),
            tracing::Level::ERROR => tracing::event!(tracing::Level::ERROR, "{}", &msg),
        }
    }
}

impl testcontainers::core::logs::consumer::LogConsumer for TracingConsumer {
    fn accept<'a>(&'a self, record: &'a testcontainers::core::logs::LogFrame) -> BoxFuture<'a, ()> {
        async move {
            match record {
                testcontainers::core::logs::LogFrame::StdOut(bytes) => self.log_bytes(bytes),
                testcontainers::core::logs::LogFrame::StdErr(bytes) => self.log_bytes(bytes),
            }
        }
        .boxed()
    }
}
