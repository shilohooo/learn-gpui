use gpui_kit::*;

/// # 可复用的技能标签组件
pub fn skill_tag(name: String) -> impl IntoElement {
    div()
        // 水平内边距
        .px_3()
        // 垂直内边距
        .py_1()
        .rounded_md()
        .bg(rgb(0xDBEAFE))
        .text_color(rgb(0x1D4ED8))
        .child(name.to_owned())
}
