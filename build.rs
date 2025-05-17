fn main() {
    let output = std::process::Command::new("docker")
        .args(["build", "-t", "zero2prod:build", "-f", "Dockerfile", "--progress", "plain", "."])
        .output()
        .expect("Failed to build docker image");
    assert!(output.status.success(), "Failed to build docker image: {}", String::from_utf8_lossy(&output.stderr));
}
