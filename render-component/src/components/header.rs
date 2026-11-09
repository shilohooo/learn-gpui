use gpui_kit::*;

/// # 标题组件
pub fn header() -> impl IntoElement {
    div()
        .text_2xl()
        .font_weight(FontWeight::BOLD)
        .italic()
        .underline()
        .child("Developer Profile")
}
