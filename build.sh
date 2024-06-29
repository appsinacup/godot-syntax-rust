cargo fmt -- --config-path rustfmt.toml
cargo clippy --fix --allow-dirty
cargo build --release
cp target/release/librust_syntax.dylib demo/addons/rust-syntax/bin/librust_syntax.macos.framework/librust_syntax.macos.dylib
