# Constellate — a real macOS app with Tauri

This is your calm task app wrapped in **Tauri v2** — a real, native macOS app (Rust +
system webview, tiny and fast, no Electron). Unlike Pake, you own the whole shell, so it
includes **real macOS window vibrancy** (the frosted-glass effect).

This README is written to *teach*. Each file is explained so you understand what a Tauri
app actually is, not just how to run it.

---

## The mental model

A Tauri app has two halves:

```
constellate/
├── package.json            ← runs the Tauri CLI (npm scripts)
├── src/                    ← THE WEB HALF (your UI — plain HTML/CSS/JS)
│   └── index.html          ← your whole app, unchanged from the browser version
└── src-tauri/              ← THE NATIVE HALF (Rust shell)
    ├── tauri.conf.json     ← describes the app + window (size, transparency, bundle)
    ├── Cargo.toml          ← Rust dependencies (incl. the vibrancy crate)
    ├── build.rs            ← Tauri's build hook (don't touch)
    ├── capabilities/
    │   └── default.json    ← which native powers the web layer is allowed to use
    └── src/
        ├── main.rs         ← desktop entry point (tiny — just calls run())
        └── lib.rs          ← the real setup: applies vibrancy to the window
```

**The key idea:** your UI is *just a web page*. Tauri loads it into a native window. Any
"native" behavior (window styling, vibrancy, file access) is configured in `src-tauri/`.
That's the part Pake hid from you — here you control it.

### What each src-tauri file does

- **`tauri.conf.json`** — the main config. `frontendDist: "../src"` tells Tauri "my UI is
  the static files in `src/`, no dev server needed." The `windows` block sets size and, crucially,
  `"transparent": true` + `"macOSPrivateApi": true` — both required for vibrancy to work.
  `titleBarStyle: "Overlay"` + `hiddenTitle` gives the clean, title-bar-less Mac look while
  keeping the traffic-light buttons.
- **`Cargo.toml`** — Rust's package.json. The line that matters for you is
  `window-vibrancy = "0.6"`, the crate that applies the frost.
- **`src/lib.rs`** — the one file you'd actually edit. On startup it grabs the window and calls
  `apply_vibrancy(...)`. Change `NSVisualEffectMaterial::HudWindow` to `Sidebar` or
  `UnderWindowBackground` to try different frost styles.
- **`main.rs`** — leave it alone; it just calls `run()` from lib.rs.
- **`capabilities/default.json`** — Tauri v2 is secure-by-default: the web layer can't do
  native things unless allowed here. We allow window dragging/minimize/close.

---

## One-time setup

You need three things installed. Check each; install only what's missing.

```bash
# 1. Xcode Command Line Tools (Apple's build toolchain)
xcode-select --install

# 2. Rust (the native layer). Installs `cargo`, Rust's build tool.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#    then restart your terminal, or run: source "$HOME/.cargo/env"

# 3. Node — you already have it (that's how you ran the earlier version).
```

Verify:

```bash
cargo --version   # should print something like: cargo 1.8x.x
node -v
```

---

## Run it

From inside this project folder:

```bash
# Install the Tauri CLI (reads package.json)
npm install

# Generate app icons (required before the first build).
# Tauri v2 accepts an SVG directly — a star-on-navy-sky icon is included:
npx tauri icon icon-source.svg
# (this writes all the sizes into src-tauri/icons/)

# Live-develop: opens a real app window; edits to src/index.html reload live.
npm run dev

# Build a distributable app + DMG (output in src-tauri/target/release/bundle/)
npm run build
```

> **First run is slow.** Rust compiles everything from scratch the first time — expect
> a few minutes. After that it's cached and fast. This is normal, not an error.

After `npm run build`, your app lands in:
```
src-tauri/target/release/bundle/macos/Constellate.app
src-tauri/target/release/bundle/dmg/Constellate_1.0.0_aarch64.dmg
```
Drag `Constellate.app` to `/Applications` and you're done.

---

## The vibrancy — where it lives and how to tune it

Open `src-tauri/src/lib.rs`. The whole effect is this call:

```rust
apply_vibrancy(
    &window,
    NSVisualEffectMaterial::HudWindow,     // ← the frost style
    Some(NSVisualEffectState::Active),      // keep blur even when unfocused
    Some(12.0),                             // corner radius
)
```

The web side cooperates via `src/index.html`: the page background is transparent (so the OS
frost shows through) while the cards stay readable. `lib.rs` also forces a light appearance so
the frost never goes dark, even in macOS dark mode. If you ever want a plain solid window,
set `"transparent": false` in `tauri.conf.json` and give `body` a solid background.

**Materials to try** (just change the one word):
- `HudWindow` — soft neutral frost (current)
- `Sidebar` — like Finder/Mail sidebars
- `UnderWindowBackground` — subtle, lets more color through
- `FullScreenUI` — heavier blur

---

## Your AI key

Same as before: open Settings (top-right, or ⌘,) in the app and paste your free Gemini key.
It's stored locally on this Mac. The app calls Gemini directly (fine for a personal
single-user app). Pressing Enter in the capture bar sends your note to Gemini, which writes
the title, tags, due date, and a short note for you. The **+** button opens a manual builder
if you'd rather fill in the details yourself. Without a key, Enter just saves your note as
typed and built-in step drafts kick in.

---

## Common first-run issues

- **`cargo: command not found`** → Rust didn't get onto your PATH. Run
  `source "$HOME/.cargo/env"` or restart the terminal.
- **Build fails on icons** → run the `npx tauri icon <png>` step above.
- **"app is damaged / unidentified developer"** when opening the built app → it's unsigned
  (normal for personal apps). Right-click the app → Open, or run
  `xattr -cr /Applications/Constellate.app`.
- **Window is fully see-through / hard to read** → the cards use their own translucent
  backgrounds; if they vanish, check you didn't remove the card background styles in
  `src/index.html`.

I built this to run, but I can't compile Rust in my environment — if any step errors, paste
the output and I'll walk you through the fix.
