use anyhow::Context;

#[derive(Eq, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub routing: Routing,
}

#[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Routing {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
}

impl Default for Routing {
    fn default() -> Self {
        Self { host: "0.0.0.0".to_string(), port: 8000 }
    }
}

impl Settings {
    pub fn new(path: Option<&str>) -> anyhow::Result<Self> {
        let env = config::Environment::with_prefix("z2p").separator("_");
        match path {
            Some(path) => config::Config::builder()
                .add_source(config::File::with_name(path))
                .add_source(env)
                .build()
                .and_then(|config| config.try_deserialize())
                .with_context(|| format!("Failed to load configuration file at {path}")),
            None => Ok(config::Config::builder()
                .add_source(config::File::with_name("zero2prod").required(false))
                .add_source(env)
                .build()
                .and_then(|config| config.try_deserialize())
                .unwrap_or_default()),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.routing.host, self.routing.port)
    }
}

#[cfg(feature = "fixtures")]
pub mod fixtures {
    use super::{Routing, Settings};

    #[rstest::fixture]
    pub fn valid() -> tempfile::NamedTempFile {
        let f = tempfile::NamedTempFile::with_suffix(".yaml").expect("Failed to create temporary file");
        let settings = Settings { routing: Routing { host: "0.0.0.0".to_string(), port: 8080 } };
        serde_yaml::to_writer(&f, &settings).expect("Failed to write settings");
        f
    }

    #[rstest::fixture]
    pub fn invalid() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::with_suffix(".yaml").expect("Failed to create temporary file")
    }
}

#[cfg(test)]
mod test {
    use crate::logs::fixtures::*;
    use sealed_test::prelude::*;

    use super::Settings;
    use super::fixtures::*;

    #[rstest::rstest]
    fn configuration_valid(_logs: (), valid: tempfile::NamedTempFile) {
        Settings::new(Some(valid.path().to_string_lossy().as_ref())).expect("Failed to load valid configuration");
    }

    #[rstest::rstest]
    fn configuration_invalid(_logs: (), invalid: tempfile::NamedTempFile) {
        Settings::new(Some(invalid.path().to_string_lossy().as_ref()))
            .expect_err("Invalid configuration should fail to load");
    }

    #[rstest::rstest]
    #[sealed_test(env = [("Z2P_ROUTING_HOST", "localhost")])]
    fn configuration_from_env_simple(_logs: ()) {
        let settings = Settings::new(None).expect("Failed to load settings");
        assert_eq!(&settings.routing.host, "localhost");
    }

    #[rstest::rstest]
    #[sealed_test(env = [("Z2P_ROUTING_HOST", "localhost")])]
    fn configuration_from_env_override(_logs: (), valid: tempfile::NamedTempFile) {
        let settings = Settings::new(Some(valid.path().to_string_lossy().as_ref())).expect("Failed to load settings");
        assert_eq!(&settings.routing.host, "localhost");
    }
}
