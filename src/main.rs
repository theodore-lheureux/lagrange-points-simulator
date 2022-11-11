use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            w.with_resizable(true)
                .with_inner_size(dioxus::desktop::wry::application::dpi::LogicalSize::new(
                    1280.0, 720.0,
                ))
                .with_min_inner_size(dioxus::desktop::wry::application::dpi::LogicalSize::new(
                    1280.0, 720.0,
                ))
        })
    });
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "hello world!"
        }
    })
}
