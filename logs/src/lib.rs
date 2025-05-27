#[cfg(feature = "fixtures")]
mod consummer;

#[cfg(feature = "fixtures")]
pub use consummer::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init<const TESTING: bool>() {
    let env = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();

    if TESTING {
        let _ = tracing_subscriber::fmt().compact().without_time().with_test_writer().with_env_filter(env).try_init();
    } else {
        let fmt = tracing_subscriber::fmt::layer().compact().without_time();
        let level = env.max_level_hint().unwrap();
        let target = tracing_subscriber::filter::Targets::new().with_target("zero2prod", level);
        let _ = tracing_subscriber::registry().with(fmt).with(target).with(env).try_init();
    }
}

#[cfg(feature = "fixtures")]
pub mod fixtures {
    use super::init;

    #[rstest::fixture]
    pub fn logs() {
        init::<true>();
    }
}
