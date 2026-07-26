// Desktop entry point. Keep this tiny — all real setup lives in lib.rs so the
// same code path is shared with mobile builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    constellate_lib::run()
}
