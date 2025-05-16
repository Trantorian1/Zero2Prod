#[derive(Eq, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub routing: Routing,
}

#[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Routing {
    pub host: String,
    pub port: u16,
}

impl Default for Routing {
    fn default() -> Self {
        Self { host: "0.0.0.0".to_string(), port: 8000 }
    }
}

impl Settings {
    pub fn new(path: Option<String>) -> Self {
        config::Config::builder()
            .add_source(config::File::with_name(path.as_deref().unwrap_or("zero2prod")))
            .build()
            .and_then(|config| config.try_deserialize())
            .unwrap_or_default()
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.routing.host, self.routing.port)
    }
}

pub mod fixtures {
    use super::Settings;

    #[rstest::fixture]
    pub fn valid() -> tempfile::NamedTempFile {
        let f = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
        serde_yaml::to_writer(&f, &Settings::default()).expect("Failed to write settings");
        f
    }

    #[rstest::fixture]
    pub fn invalid() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new().expect("Failed to create temporary file")
    }
}

#[cfg(test)]
mod test {
    use super::Settings;
    use super::fixtures::*;

    #[rstest::rstest]
    fn configuration_valid(valid: tempfile::NamedTempFile) {
        Settings::new(Some(valid.path().to_string_lossy().to_string()));
    }

    #[rstest::rstest]
    fn configuration_invalid(invalid: tempfile::NamedTempFile) {
        Settings::new(Some(invalid.path().to_string_lossy().to_string()));
    }
}
