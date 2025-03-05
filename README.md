![Static Badge](https://img.shields.io/badge/Discord-Discord-blue?style=for-the-badge&logo=discord&label=Contact&labelColor=white&color=7289da)

![X (formerly Twitter) URL](https://img.shields.io/twitter/url?url=https%3A%2F%2Fx.com%2Felhaban3ro&style=social&label=Follow%20Me%20on%20X)

# Welcome to GameTools 🦀

## Introduction 🎯

Driven by my desire to learn a new programming language, I chose Rust and developed GameTools—an efficient and lightweight solution I had been seeking for quite some time.

GameTools is built entirely in **Rust** and can run on any system that supports it. Please note that **GameTools** is a **command-line application**, designed for speed and efficiency.

### Features 🛠️

Currently, GameTools offers two main features:

1. **Anti-AFK**: Default command: ***Left Control + P***
2. **Auto-Run**: Default command: ***Left Control + K***

These settings can be customized; see the [Configuration](#configuration) section.

---

## Downloads 📥

If you're not interested in modifying or contributing to this project, you can download the latest [release](https://github.com/ElHaban3ro/GameTools/releases/tag/Release) or the latest version of GameTools for your platform:

- [Linux x86-64](https://github.com/ElHaban3ro/GameTools/releases/download/Release/GameTools-Linux-x86_64)
- Windows (coming soon)

**Note:** On **Linux**, you may need to install or update the **x11** library, which enables interaction with the system's graphical environment.

---

## Compilation 🔧

If an executable is unavailable for your platform or you've modified the code, you can build the application using the following command (assuming you have Rust installed):

```sh
cargo build --release
```

---

## Configuration ⚙️

GameTools generates a JSON configuration file named `configs.json` in the same directory as the executable. By default, it contains:

```json
{
    "macro_start_afk": [
        "ControlLeft",
        "KeyP"
    ],
    "macro_stop_afk": [
        "ControlLeft",
        "KeyI"
    ],
    "macro_start_auto_run": [
        "ControlLeft",
        "KeyK"
    ],
    "macro_stop_auto_run": [
        "ControlLeft",
        "KeyL"
    ],
    "run_key": "ShiftLeft"
}
```

Each key represents a **command**, and its value (typically an array) defines the **shortcut** to trigger that command. The `run_key` is used specifically for auto-run functionality, such as holding down the run key in games like Minecraft, where Shift isn't the default run key.

You can use the same shortcut for both **activation** and **deactivation**, but a separate deactivation shortcut is supported for **accessibility** reasons.

### Example Customization ✏️

To activate the `macro_start_afk` command using `Ctrl + Shift + A + Z`, modify the JSON file as follows:

```json
{
    "macro_start_afk": [
        "ControlLeft",
        "ShiftLeft",
        "KeyA",
        "KeyZ"
    ]
}
```

---

## Need Help? 💬

For any questions, feel free to contact me via [**Discord**](https://discord.gg/gs2FPECRNg).