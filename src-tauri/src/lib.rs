// This is the heart of the native shell. On startup we grab the main window,
// force a dark appearance to match the night-sky UI, and apply a subtle macOS
// vibrancy tint behind the web layer so the rounded window feels native and
// there's no white flash on load. Everything else — the UI, storage, the AI
// call — lives in the web layer (src/index.html).

use tauri::Manager;

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the window we declared as "main" in tauri.conf.json.
            let window = app.get_webview_window("main").unwrap();

            // The app is a dark night sky, so force a dark appearance. This keeps
            // native bits (scrollbars, menus, the vibrancy tint) dark regardless
            // of the user's macOS light/dark setting.
            let _ = window.set_theme(Some(tauri::Theme::Dark));

            // A dark, low-key vibrancy sits behind the transparent web layer. The
            // canvas paints the full starfield on top, so this mostly matters at
            // load (no white flash) and at the rounded window edges, where it
            // gives a soft native frost. UnderWindowBackground is the subtlest
            // dark material; try ::HudWindow or ::FullScreenUI for more blur.
            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::UnderWindowBackground,
                Some(NSVisualEffectState::Active),
                Some(14.0), // corner radius, matches the app's rounded feel
            )
            .expect("vibrancy is only supported on macOS 10.10+");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the Constellate app");
}
