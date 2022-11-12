use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Menu(cx: Scope) -> Element {
    let cog_icon = include_str!("../assets/icons/cog.svg");
    let show_menu = use_state(&cx, || "hide");

    cx.render(rsx!(
        style { [include_str!("../assets/style/menu.css")] },
        div {
            div {
                class: "menu-button",
                onclick: move |_| {
                    show_menu.modify(|v| {
                        if *v == "hide" {
                            ""
                        } else {
                            "hide"
                        }
                    });
                },
                dangerous_inner_html: "{cog_icon}",
            }
            div {
                class: "menu {show_menu}",
            }
        }
    ))
}
