use gpui_kit::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child("Hello, World!")
            .child("My first Rust desktop app")
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| HelloWorld))
            .expect("Failed to open window");
    });
}
