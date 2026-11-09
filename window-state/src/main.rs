use gpui_kit::*;

struct LearningApp {
    // 视图状态
    count: usize,
}

impl Render for LearningApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xF8FAFC))
            .text_color(rgb(0x0F172A))
            .child(div().text_2xl().child("GPUI Learning"))
            .child(div().text_color(rgb(0x64748B)).child("Week 1 - Day 2"))
            // 递增按钮
            .child(
                div()
                    // 内边距
                    .p_4()
                    // 圆角
                    .rounded_lg()
                    .bg(rgb(0xDBEAFE))
                    // 鼠标指针样式
                    .cursor_pointer()
                    // 展示状态（数据）
                    .child(format!("Count: {}", self.count))
                    // 事件处理：鼠标点下去
                    .on_mouse_down(
                        // 绑定鼠标左键，它在鼠标按下时触发
                        MouseButton::Left,
                        // 事件监听器（回调处理）
                        // 它将事件回调与当前 LearningApp Entity 关联，让回调能够安全地访问该 Entity 的状态
                        // this = 当前视图实例
                        // _event = 鼠标事件对象，包含事件相关信息
                        // _window = 当前窗口对象
                        // cx = 当前 Entity 上下文
                        cx.listener(|this, _event, _window, cx| {
                            println!("触发递增事件 - MouseButton::Left");
                            this.count += 1;
                            // 通知 GPUI 当前 Entity 的视图需要更新
                            // 注意：notify() 不等于立即同步绘制屏幕，它通知框架安排相应的更新
                            cx.notify();
                        }),
                    ),
            )
            // TODO - 递减按钮，点击后计数减 1，且不能小于 0
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0xDBEAFE))
                    .cursor_pointer()
                    .child("Decrease")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            println!("触发递减事件");
                            if this.count == 0 {
                                println!("统计数字为 0 ，跳过不处理");
                                return;
                            }
                            this.count -= 1;
                            cx.notify();
                        }),
                    ),
            )
            // TODO - Double 按钮，点击后计数乘 2
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0x099FFF))
                    .cursor_pointer()
                    .child(div().text_color(rgb(0xFFFFFF)).child("Double"))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            println!("触发翻倍事件");
                            this.count *= 2;
                            cx.notify();
                        }),
                    ),
            )
            // 重置按钮
            .child(
                div()
                    .p_3()
                    .rounded_md()
                    .bg(rgb(0xE2E8F0))
                    .cursor_pointer()
                    .child("Reset")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            println!("触发重置事件");
                            this.count = 0;
                            cx.notify();
                        }),
                    ),
            )
    }
}

/// # GPUI - 窗口状态
fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        let (handle, entity) = open_window(WindowOptions::default(), cx, |window, cx| {
            // 设置窗口标题
            window.set_window_title("Rust GPUI 学习系统");

            cx.new(|_| LearningApp { count: 10 })
        })
        .expect("Failed to open window");
    });
}
