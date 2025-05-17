use testcontainers::{ImageExt, core::ports::IntoContainerPort as _, runners::SyncRunner};
use zero2prod::configuration;

pub struct App {
    _container: testcontainers::Container<testcontainers::GenericImage>,
    address: String,
    client: reqwest::Client,
}

impl App {
    fn new(
        settings: configuration::Settings,
        container: testcontainers::Container<testcontainers::GenericImage>,
    ) -> Self {
        let port = container.get_host_port_ipv4(settings.routing.port).unwrap();
        let host = container.get_host().unwrap();
        let address = format!("http://{host}:{port}");

        Self { _container: container, address, client: reqwest::Client::new() }
    }

    #[allow(unused)]
    pub fn health_check(&self) -> reqwest::RequestBuilder {
        self.client.get(format!("{}/health_check", self.address))
    }

    #[allow(unused)]
    pub fn subscriptions(&self) -> reqwest::RequestBuilder {
        self.client.post(format!("{}/subscriptions", self.address))
    }
}

#[rstest::fixture]
pub fn app(#[default(None)] f: Option<tempfile::NamedTempFile>) -> App {
    let (app, settings) = match f {
        Some(f) => {
            let path = f.path().to_string_lossy();
            let settings = configuration::Settings::new(Some(&path)).expect("Failed to load settings");
            let mount = testcontainers::core::Mount::bind_mount(path, "/app/zero2prod.yml");
            let app = testcontainers::GenericImage::new("zero2prod", "build")
                .with_exposed_port(settings.routing.port.tcp())
                .with_mount(mount)
                .start()
                .expect("Failed to start app");
            (app, settings)
        }
        None => {
            let settings = configuration::Settings::new(None).expect("Failed to load settings");
            let app = testcontainers::GenericImage::new("zero2prod", "build")
                .with_exposed_port(settings.routing.port.tcp())
                .start()
                .expect("Failed to start app");
            (app, settings)
        }
    };

    App::new(settings, app)
}
