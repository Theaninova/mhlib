[package]
name = "mhjnr"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mhjnr-viewer"
path = "src/main.rs"

[lib]
name = "mhjnr"
crate-type = ["cdylib", "lib"]

[dependencies]
itertools = "0.10.5"
unicode-segmentation = "1.10.1"
encoding_rs = "0.8.32"
binrw = "0.11.1"
serde = {version = "1.0.160", features = ["derive"]}
serde-xml-rs = "0.6.0"
image = "0.24.6"
base64 = "0.21.0"
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }