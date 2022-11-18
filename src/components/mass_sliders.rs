use dioxus::prelude::*;

use super::mass::{MASS_MASSES, MASS_NAMES, MASS_VELOCITIES};

#[allow(non_snake_case)]
pub fn MassSliders(cx: Scope, mass_id: u32) -> Element {
    let mass_names = use_atom_ref(&cx, MASS_NAMES).write();
    let mass_masses = use_atom_ref(&cx, MASS_MASSES).write();
    let mass_velocities = use_atom_ref(&cx, MASS_VELOCITIES).write();

    cx.render(rsx! {
        div {
            "ksdflkdjflkj"
        }
    })
}
