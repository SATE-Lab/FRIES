macro_rules! cargo_toml_template {
    ($name:expr, $edition:expr) => {
        format_args!(
            r##"[package]
name = "rust-fuzzer"
version = "0.0.0"
publish = false
{edition}
[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = "0.4"

[dependencies.{name}]
path = ".."

# Prevent this from interfering with workspaces
[workspace]
members = ["."]

[profile.release]
debug = 1
"##,
            name = $name,
            edition = if let Some(edition) = &$edition {
                format!("edition = \"{}\"\n", edition)
            } else {
                String::new()
            },
        )
    };
}
