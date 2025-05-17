fn main() {
    println!("cargo::rerun-if-changed=src/");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Dockerfile");
    let output = std::process::Command::new("docker")
        .args(["build", "-t", "zero2prod:build", "-f", "Dockerfile", "--progress", "plain", "."])
        .output()
        .expect("Failed to build docker image");
    assert!(output.status.success(), "Failed to build docker image: {}", String::from_utf8_lossy(&output.stderr));
}
