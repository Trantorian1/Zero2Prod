fn main() {
    println!("cargo::rerun-if-changed=src/");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Dockerfile");
    let output = std::process::Command::new("docker")
        .args(["build", "-t", "zero2prod:build", "-f", "Dockerfile", "--progress", "plain", "."])
        .output()
        .expect("Failed to build docker image");
    if !output.status.success() {
        println!("cargo::warning=Failed to build docker image!");
    }
}
