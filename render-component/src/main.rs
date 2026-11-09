mod app;
mod components;

use app::DeveloperApp;
use gpui_kit::*;

/// # GPUI - Render、IntoElement 与声明式 UI
fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |window, cx| {
            window.set_window_title("GPUI Render & Components");
            cx.new(|_| DeveloperApp)
        })
        .expect("Failed to open window");
    });
}
