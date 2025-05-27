fn main() {
    println!("cargo::rerun-if-changed=src/");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=../Cargo.toml");
    println!("cargo::rerun-if-changed=../Cargo.lock");
    println!("cargo::rerun-if-changed=../Dockerfile");
    println!("cargo::rerun-if-env-changed=RUST_TEST");

    if let Err(std::env::VarError::NotPresent) = std::env::var("RUST_TEST") {
        return;
    }

    let output = std::process::Command::new("docker")
        .args(["build", "-t", "zero2prod:build", "-f", "../Dockerfile", "--progress", "plain", "../"])
        .output()
        .expect("Failed to build docker image");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).replace("\n", "\ncargo::warning=");
        println!("cargo::warning=Failed to build docker image!\n{err}");
    }
}
