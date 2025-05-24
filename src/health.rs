use zero2prod::configuration;

fn main() {
    let settings = configuration::Settings::new(None).expect("Failed to load configuration");
    let host = settings.routing.host;
    let port = settings.routing.port;

    let req = reqwest::blocking::get(format!("http://{host}:{port}/health_check")).unwrap();
    assert!(req.status().is_success());
}
