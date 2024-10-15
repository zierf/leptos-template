# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) +
[Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Format view! Macros

```SH
leptosfmt src/**/*.rs
```

## Serve Browser

```SH
trunk serve
```

Open [http://localhost:1420](http://localhost:1420/) to test the app.

## Serve Tauri-App

```SH
cargo tauri dev
```

## Build Web-App

```SH
trunk build --release
```

Built files are stored in the `dist` folder.


## Build Tauri-App

```SH
cargo tauri build
```
