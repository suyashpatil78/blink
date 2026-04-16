# Blink

A small, fast **desktop application launcher** for Linux. Press **Ctrl+Space**, type a few letters of an app name, hit **Enter** to launch the top match, or pick from the list. Built with [Tauri 2](https://v2.tauri.app/) and [Angular](https://angular.dev/) so the UI stays light while the heavy lifting stays on the system side.

---

## Why Blink?

- **Stays out of the way** until you summon it: borderless, transparent window that does not clutter the taskbar.
- **Knows your Linux apps** by scanning standard Freedesktop locations: system packages, **Flatpak**, **Snap**, and `~/.local/share/applications`.
- **Sensible matching**: prefix matches rank above substring matches; results are capped for snappy typing.
- **Native launch** via `gio launch`, so behavior matches what your desktop environment expects.

---

## Requirements for development (Linux)

You will need:

| Tool | Notes |
|------|--------|
| **Node.js** | A current LTS (e.g. 20+) works well with the Angular toolchain in this repo. |
| **Rust** | Stable toolchain via [rustup](https://rustup.rs/). |
| **System libraries** | WebKitGTK and friends for Tauri. Install the packages your distro documents for **Tauri v2**; see the [official Linux prerequisites](https://v2.tauri.app/start/prerequisites/#linux). |

**Runtime** (for launching apps): `gio` from **GLib** is used to open `.desktop` files. On Debian/Ubuntu this is typically in `libglib2.0-bin`, which the packaged `.deb` already lists as a dependency.

---

## Quick start (development)

From the repository root:

```bash
npm install
npm run tauri dev
```

This runs the Angular dev server on port **1420** and opens the Tauri shell. Use **Ctrl+Space** to show the launcher (registering the global shortcut may require a graphical session with appropriate permissions).

---

## Installation

- Download the `.deb` file from [here](https://github.com/suyashpatil78/blink/releases/download/v0.1.0/blink.deb). You can also check other releases in the release section.

- Install it with your package manager, for example:

```bash
sudo apt install ./blink.deb
```

Adjust the filename to match with your downloaded file.

---

## Using Blink

| Action | What it does |
|--------|----------------|
| **Ctrl+Space** | Show and focus the launcher. |
| **Type** | Filter installed applications by name (case-insensitive). |
| **Enter** | Launch the first suggestion. |
| **Click** a row | Launch that application. |
| **Escape** | Hide the launcher and clear the query. |

If **Ctrl+Space** is already bound elsewhere (another launcher, your IDE, or the window manager), change the shortcut in your settings.

- Go to Keyboard/Shortcuts.
- Add custom shortcut.
- In command/path add -> `/usr/bin/blink` and type the keybinding which you want to set.

---

## Project layout

| Path | Role |
|------|------|
| `src/` | Angular frontend: search UI and Tauri `invoke` calls. |
| `src-tauri/` | Rust backend: desktop entry index, search, `gio launch`, window show/hide, global shortcut. |

