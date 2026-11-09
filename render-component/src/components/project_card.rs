use crate::components::skill_tag::skill_tag;
use gpui_kit::*;

pub struct ProjectInfo {
    pub name: String,
    pub stacks: Vec<String>,
    pub desc: String,
}

pub fn project_card(project_info: ProjectInfo) -> impl IntoElement {
    div()
        .rounded_md()
        .bg(rgb(0xF8FAFC))
        .p_4()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_xl().child(project_info.name))
        .child(
            div().flex().flex_wrap().gap_2().children(
                project_info
                    .stacks
                    .into_iter()
                    .map(|stack| skill_tag(stack)),
            ),
        )
        .child(div().text_xs().child(project_info.desc))
}
