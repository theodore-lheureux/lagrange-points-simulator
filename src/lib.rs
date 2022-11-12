use dioxus::prelude::*;

use crate::components::menu::Menu;

pub mod components;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("./assets/style/global.css")] },
        div {
            Menu {}
        }
    ))
}
