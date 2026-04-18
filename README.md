<div align="center">

<img src="src-tauri/icons/128x128.png" width="128" height="128" alt="Blink" />

### A lightweight, fast desktop application launcher for Linux

<p>
  <a href="https://github.com/suyashpatil78/blink/releases">
    <img src="https://img.shields.io/badge/release-v0.3.0-blue" alt="Release" />
  </a>

  <a href="https://v2.tauri.app">
    <img src="https://img.shields.io/badge/stack-Tauri%202-26A8A6?style=flat-square&logo=tauri&logoColor=white" alt="Tauri" />
  </a>

  <a href="https://angular.dev">
    <img src="https://img.shields.io/badge/UI-Angular%2020-DD0031?style=flat-square&logo=angular&logoColor=white" alt="Angular" />
  </a>

  <img src="https://img.shields.io/badge/platform-linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux" />
</p>

<br />

</div>

## Overview

---

**Blink** is a minimal launcher for Linux: press **Ctrl+Space**, type part of an app name, and launch from the list. It stays out of the way—borderless, transparent, no taskbar entry until you need it—while indexing Freedesktop **`.desktop`** entries from system paths, **Flatpak**, **Snap**, and **`~/.local/share/applications`**. Matching prefers names that **start** with your query, then **contains**; launches go through **`gio launch`** so behavior matches your desktop environment. It also supports searching Google with **`g <query>`**.

Built with **[Tauri 2](https://v2.tauri.app/)** and **[Angular](https://angular.dev/)**.

---

## Highlights

- **Global shortcut** — show or focus the overlay with **Ctrl+Space** (configurable in code).
- **Fast search** — debounced query against a prebuilt index of installed applications.
- **Packaged for Debian** — `.deb` with icons under **`hicolor`** and a **`.desktop`** entry for your app menu.
- **Google search** — search Google with **`g <query>`**.

---

## Preview


https://github.com/user-attachments/assets/4cc7b618-c8d9-4258-bb3e-c06423c9fc46

## Screenshots

<img width="877" height="528" alt="Screenshot from 2026-04-18 12-24-30" src="https://github.com/user-attachments/assets/4d487914-d21d-4a5b-874f-1efc4a2e8149" />
<img width="867" height="538" alt="Screenshot from 2026-04-18 12-24-15" src="https://github.com/user-attachments/assets/c6d32572-9429-464b-bdc8-345608c31ce7" />
<img width="870" height="537" alt="Screenshot from 2026-04-18 12-24-05" src="https://github.com/user-attachments/assets/6b1a4a52-12cd-4a20-a36b-a5ce3b953d67" />
<img width="1010" height="601" alt="Screenshot from 2026-04-18 12-23-53" src="https://github.com/user-attachments/assets/52a6fad8-abb5-40ea-a37f-9f563f1d5929" />



## Requirements (development on Linux)

| Tool | Notes |
|------|--------|
| **Node.js** | Current LTS (e.g. 20+) for the Angular toolchain. |
| **Rust** | Stable toolchain via [rustup](https://rustup.rs/). |
| **System libraries** | WebKitGTK and related packages for Tauri — see [Tauri v2 Linux prerequisites](https://v2.tauri.app/start/prerequisites/#linux). |

**Runtime** for launching apps: **`gio`** (usually **`libglib2.0-bin`** on Debian/Ubuntu); the packaged `.deb` declares this dependency.

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

- Download the `.deb` file from [here](https://github.com/suyashpatil78/blink/releases/download/v0.3.0/blink_0.3.0_amd64.deb). You can also check other releases in the release section.

- Install it with your package manager, for example:

```bash
sudo apt install ./blink_0.3.0_amd64.deb
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

