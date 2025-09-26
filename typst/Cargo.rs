// Cargo.toml
[package]
name = "typst-math"
version = "0.1.0"
edition = "2021"

[dependencies]
unicode-math = "0.1"
unicode-segmentation = "1.0"
euclid = "0.22"
lyon = "1.0"
svg = "0.10"
image = "0.24"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
logos = "0.13"
once_cell = "1.18"
smallvec = "1.11"

[dev-dependencies]
criterion = "0.5"