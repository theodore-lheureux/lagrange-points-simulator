use dioxus::prelude::*;

use crate::components::mass::{MASS_VELOCITIES, MASS_MASSES, MASS_NAMES};


#[allow(non_snake_case)]
#[inline_props]
pub fn MassSliders(cx: Scope, id: u32) -> Element {
    let name = use_atom_ref(&cx, MASS_NAMES)
        .read()
        .get(id)
        .unwrap()
        .clone();
    let mass = use_atom_ref(&cx, MASS_MASSES)
        .read()
        .get(&id)
        .unwrap()
        .clone();
    let velocity = use_atom_ref(&cx, MASS_VELOCITIES)
        .read()
        .get(&id)
        .unwrap()
        .clone();

    cx.render(rsx! {
        div {
            "{name}"
        }
        form {
            oninput: move |evt| {
                use_atom_ref(&cx, MASS_MASSES).write().insert(*id, evt.values.get(&format!("slider{id}mass")).unwrap().clone().parse().unwrap());
                use_atom_ref(&cx, MASS_VELOCITIES).write().insert(*id, evt.values.get(&format!("slider{id}velocity")).unwrap().clone().parse().unwrap());
            },
            div {
                class: "row",
                label {
                    "for":"slider{id}mass",
                    "Mass of object"
                }
                div {
                    class: "column",
                    input {
                        id: "slider{id}mass",
                        name: "slider{id}mass",
                        class: "slider",
                        "type": "range",
                        min: "1",
                        max: "100",
                        background: "linear-gradient(to right, coral 0%, coral {mass}%, #fff {mass}%, white 100%)",
                        value: "{mass}",
                    }
                }
                p {
                    class: "value",
                    dangerous_inner_html: "{mass} kg",
                }
            }
            div {
                class: "row",
                label {
                    "for":"slider{id}velocity",
                    "Velocity of object"
                }
                div {
                    class: "column",
                    input {
                        id: "slider{id}velocity",
                        class: "slider",
                        name: "slider{id}velocity",
                        "type": "range",
                        min: "1",
                        max: "100",
                        background: "linear-gradient(to right, coral 0%, coral {velocity}%, #fff {velocity}%, white 100%)",
                        value: "{velocity}",
                    }
                }
                p {
                    class: "value",
                    dangerous_inner_html: "{velocity} m/s",
                }
            }
        }
    })
}
