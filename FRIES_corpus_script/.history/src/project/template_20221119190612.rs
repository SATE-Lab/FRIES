/// fuzz项目中的cargo_toml
macro_rules! cargo_toml_template {
    ($name:expr, $edition:expr) => {
        format_args!(
            r##"[package]
name = "{}-rust-fuzzer"
version = "0.0.0"
publish = false
{edition}

#存储元信息, 被fuzzer识别, 来判断是否是fuzz_dir
[package.metadata]
rust-fuzzer = true

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

/// gitignore模板
macro_rules! gitignore_template {
    () => {
        format_args!(
            r##"target
corpus
artifacts
coverage
"##
        )
    };
}
