use std::collections::HashMap;

use dioxus::prelude::*;

static MASS_ID_COUNTER: Atom<u32> = |_| 0;
pub static MASS_NAMES: AtomRef<HashMap<u32, String>> =
    |_| HashMap::from([(0, String::from("Mass 1")), (1, String::from("Mass 2"))]);
pub static MASS_MASSES: AtomRef<HashMap<u32, u32>> = |_| HashMap::from([(0, 2), (1, 4)]);
pub static MASS_VELOCITIES: AtomRef<HashMap<u32, u32>> = |_| HashMap::from([(0, 2), (1, 4)]);

pub struct Mass {
    pub name: String,
    pub mass: u32,
    pub velocity: u32,
}

impl Mass {
    pub fn build(cx: &Scope, name: String, mass: u32, velocity: u32) -> Mass {
        let id = *use_read(cx, MASS_ID_COUNTER);
        use_set(cx, MASS_ID_COUNTER)(id + 1);

        use_atom_ref(&cx, MASS_NAMES)
            .write()
            .insert(id, name.clone());

        use_atom_ref(&cx, MASS_MASSES).write().insert(id, mass);
        use_atom_ref(&cx, MASS_VELOCITIES)
            .write()
            .insert(id, velocity);

        Mass {
            name,
            mass,
            velocity,
        }
    }
}
