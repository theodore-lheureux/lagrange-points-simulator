use components::mass::Mass;
use dioxus::prelude::*;

use crate::components::menu::Menu;

pub mod components;

static MASS_LIST: AtomRef<Vec<Mass>> = |_| {
    Vec::from([
        Mass {
            mass: 20,
            velocity: 20,
        },
        Mass {
            mass: 30,
            velocity: 20,
        },
    ])
};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("./assets/style/global.css")] },
        div {
            Menu {}
        }
    ))
}
