fn main() {
    std::process::Command::new("docker")
        .args(["build", "-t", "zero2prod:build", "-f", "./Dockerfile", "--progress", "plain", "."])
        .output()
        .expect("Failed to build docker image");
}
