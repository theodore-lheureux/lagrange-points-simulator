use dioxus::prelude::*;

use crate::components::mass::MASS_NAMES;

#[allow(non_snake_case)]
pub fn Tabs(cx: Scope) -> Element {
    let mass_names = use_atom_ref(&cx, MASS_NAMES).read().clone();
    let mut mass_names: Vec<(u32, String)> = mass_names.into_iter().collect();
    mass_names.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let tabs = rsx! {
        mass_names.iter().map(|(k, v)| {
            rsx! {
                div {
                    class: "tab",
                    key: "{k}",
                    "{v}"
                }
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "tab-container",
            tabs
        }
    })
}
