use crate::components::{mass::Mass, menu::{mass_sliders::MassSliders, tabs::Tabs}};

use dioxus::prelude::*;

use super::mass::{MASS_NAMES};

pub mod tabs;
pub mod mass_sliders;

#[allow(non_snake_case)]
#[inline_props]
// , mass_names: UseRef<HashMap<u32, String>>
pub fn Menu(cx: Scope) -> Element {
    let cog_icon = include_str!("../assets/icons/cog.svg");
    let show_menu = use_state(&cx, || "hide");
    let value_slider1 = use_state(&cx, || 10);

    let mass_names = use_atom_ref(&cx, MASS_NAMES);


    let mut sorted_map: Vec<u32> = mass_names.read().clone().into_keys().collect();
    sorted_map.sort_unstable_by(|a, b| a.cmp(b));

    let slider_list = rsx! {
        sorted_map.into_iter().map(|k| rsx! {
            li {
                key: "{k}",
                MassSliders { id: k }
            }
        })
    };

    cx.render(rsx!(
        style { [include_str!("../assets/style/menu.css")] },
        div {
            class: "menu-button",
            onclick: move |_| { show_menu.modify(|v| if *v == "hide" { "" } else { "hide" })},
            dangerous_inner_html: "{cog_icon}",
        }
        div {
            class: "menu {show_menu}",
            Tabs {}
            // h1 {
            //     "Parameters :"
            // }
            div {
                class: "menu-content {show_menu}",
                div {
                    class: "row",
                    label {
                        "for":"slider1",
                        "Simulation speed"
                    }
                    div {
                        class: "column",
                        input {
                            id: "slider1",
                            class: "slider",
                            "type": "range",
                            min: "1",
                            max: "99",
                            background: "linear-gradient(to right, coral 0%, coral {value_slider1}%, #fff {value_slider1}%, white 100%)",
                            value: "{value_slider1}",
                            oninput: move |evt| {
                                value_slider1.set(evt.value.clone().parse().unwrap());
                            },
                        }
                    }
                    p {
                        class: "value",
                        dangerous_inner_html: "{value_slider1} x (not actual time)",
                    }

                }
                ul {
                    //slider_list
                }
                button {
                    onclick: move |_| { 
                        Mass::build(&cx, String::from("Mass 1"), 10, 10);
                    },
                    "Add Mass"
                }

            }
        }
    ))
}
