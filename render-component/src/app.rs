use crate::components::header::header;
use crate::components::profile_card::profile_card;
use gpui_kit::*;

pub struct DeveloperApp;

impl Render for DeveloperApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .bg(rgb(0xF8FAFC))
            .text_color(rgb(0x0F172A))
            .child(header())
            .child(profile_card("Shiloh595", "Full Stack Developer"))
            .child(profile_card("Bruce Lee", "Project Manager"))
            .child(
                div()
                    .text_color(rgb(0x64748B))
                    .child("Build with Rust + GPUI"),
            )
    }
}
