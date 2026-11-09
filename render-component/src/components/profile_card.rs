use crate::components::project_card::{ProjectInfo, project_card};
use crate::components::skill_tag::skill_tag;
use gpui_kit::*;

pub fn profile_card(name: &'static str, position: &'static str) -> impl IntoElement {
    let skills = [
        "Rust",
        "GPUI",
        "Tokio",
        "SQLite",
        "Java",
        "Vue",
        "PostgreSQL",
    ]
    .map(str::to_string);

    div()
        .flex()
        .flex_col()
        .gap_4()
        .p_4()
        .rounded_lg()
        .bg(rgb(0xFFFFFF))
        .text_color(rgb(0x0F172A))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(div().text_xl().child(name))
                .child(div().text_color(rgb(0x64748B)).child(position)),
        )
        .child(
            div().flex().flex_col().gap_2().child("Skills").child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    // children 允许一直设置多个子元素，适合用于动态渲染
                    .children(skills.into_iter().map(skill_tag)),
            ),
        )
        .child(project_card(ProjectInfo {
            name: String::from("项目名称"),
            stacks: vec![String::from("项目技术栈1"), String::from("项目技术栈2")],
            desc: String::from("项目描述"),
        }))
}
