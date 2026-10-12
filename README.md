# Pixel Buds Desktop

> You have Pixel Buds but don't want to use Google Chrome? No longer a problem with this app!

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-1.0.0-green.svg)](package.json)
[![Platform](https://img.shields.io/badge/Platform-Windows%20(WebView2)-lightgrey.svg)]()

An unofficial standalone desktop companion for Google Pixel Buds ([https://mypixelbuds.google.com](https://mypixelbuds.google.com)), built with **Tauri v2** and **Rust**.

Developed by **Renev.** ([info@renev.nl](mailto:info@renev.nl)).

---

## Features

- **Native Web Bluetooth Support:** Full Bluetooth Low Energy (BLE) integration via Microsoft Edge WebView2 on Windows.
- **Chrome User-Agent Emulation:** Properly recognized by Google Accounts and the Pixel Buds web app.
- **Frameless Window Integration:** Clean in-header window management (minimize, maximize/restore, close) matching the Google header design.
- **Optimized UI:** Automatically cleans up external store links and unnecessary clutter while preserving session management and account switching.
- **Smart Auth Redirect:** Automatically bounces sign-in completion fallbacks (`myaccount.google.com`) directly back into the companion app.

---

## Quick Start (Development)

### Requirements
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (stable toolchain)
- Microsoft Edge WebView2 (included by default on Windows 10/11)

### Run locally
```powershell
npm install
npm run dev
```

---

## Building a Release

To create standalone binaries (`.exe` and `.msi` installers) for distribution on GitHub Releases:

```powershell
npm run build
```

The resulting installers and standalone binaries will be generated in:
```
src-tauri/target/release/
src-tauri/target/release/bundle/msi/
src-tauri/target/release/bundle/nsis/
```

---

## Author & Contact

- **Creator:** Renev.
- **Email:** [info@renev.nl](mailto:info@renev.nl)

---

## License & Disclaimer

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for details.

> **Disclaimer:** Pixel Buds, Google, Chrome, and Google Accounts are trademarks of Google LLC. This project is an independent, unofficial open-source application and is not affiliated with, sponsored by, or endorsed by Google LLC.
