cargo fmt -- --config-path rustfmt.toml
cargo clippy --fix --allow-dirty
cargo build --release
cp target/release/librust_syntax.dylib demo/addons/rust-syntax/bin/librust-syntax.macos.template_debug.dylib
