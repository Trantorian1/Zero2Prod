#![cfg(feature = "fixtures")]

use futures::{FutureExt, future::BoxFuture};

pub struct TracingConsumer;

impl TracingConsumer {
    fn log_bytes(&self, bytes: &[u8]) {
        print!("{}", String::from_utf8_lossy(bytes));
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
