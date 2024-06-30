# Rust Syntax for Godot

Note: This is still WIP

<p align="center">
<img src="https://github.com/appsinacup/gdrust-syntax/blob/main/syntax.png?raw=true" style="height:'512px'; width:'auto'"/>
</p>

<p align="center">
        <img src="https://github.com/appsinacup/gdrust-syntax/actions/workflows/runner.yml/badge.svg?branch=main"
            alt="Godot Rust Syntax"></a>
        <img src="https://img.shields.io/badge/Godot-4.2-%23478cbf?logo=godot-engine&logoColor=white" />
</p>


-----

<p align = "center">
<b>Rust Syntax Support inside Godot Editor. </b>
</p>

-----

Godot Rust Syntax offers support to edit `.rs` files inside the Godot Editor. Also offers support to view warnings and errors in Rust code directly inside the Godot Editor.

# Installation

- Automatic (Recommended): Download the plugin from the official [Godot Asset Store](https://godotengine.org/asset-library/asset/2267) using the `AssetLib` tab in Godot:
    - TODO

- Manual: Download the [latest github release](https://github.com/appsinacup/gdrust-syntax/releases/latest) and move only the `addons` folder into your project `addons` folder.

# Platforms

- Windows (x86_64, x86_32)
- macOS (x86-64 + arm64 Universal)
- Linux (x86_64)

# How to build

Requirements:

- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

1. Update dependencies to latest:

```bash
cargo update
```

2. Build the project
```bash
cargo build --release
```


3. Copy the output to bin folder of the addon:

Eg. macOS
```bash
cp target/release/librust_syntax.dylib demo/addons/rust-syntax/bin/librust_syntax.macos.framework/librust_syntax.macos.dylib
```

For the correct path to use inside the bin folder, look inside the `demo/addons/rust-syntax.gdextension`.
