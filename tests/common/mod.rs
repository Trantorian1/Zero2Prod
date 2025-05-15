use testcontainers::{core::ports::IntoContainerPort as _, runners::SyncRunner};

pub struct App {
    _container: testcontainers::Container<testcontainers::GenericImage>,
    address: String,
    client: reqwest::Client,
}

impl App {
    fn new(container: testcontainers::Container<testcontainers::GenericImage>) -> Self {
        let port = container.get_host_port_ipv4(8000).unwrap();
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
pub fn app() -> App {
    let app = testcontainers::GenericImage::new("zero2prod", "build")
        .with_exposed_port(8000.tcp())
        .start()
        .expect("Failed to start container");

    App::new(app)
}
