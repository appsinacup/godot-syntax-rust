cargo fmt -- --config-path rustfmt.toml
cargo clippy --fix --allow-dirty
cargo build
mkdir -p demo/addons/godot_syntax_rust/bin/libgodot_rust_syntax.macos.framework/
cp target/debug/libgodot_rust_syntax.dylib demo/addons/godot_syntax_rust/bin/libgodot_rust_syntax.macos.framework/libgodot_rust_syntax.macos.dylib
