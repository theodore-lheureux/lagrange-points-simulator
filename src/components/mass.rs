use std::collections::HashMap;

use dioxus::prelude::*;

use super::menu::MenuProps;

static MASS_ID_COUNTER: Atom<u32> = |_| 2;
pub static MASS_NAMES: AtomRef<HashMap<u32, String>> =
    |_| HashMap::new();
pub static MASS_MASSES: AtomRef<HashMap<u32, u32>> = |_| HashMap::new();
pub static MASS_VELOCITIES: AtomRef<HashMap<u32, u32>> = |_| HashMap::new();

pub struct Mass {
    pub name: String,
    pub mass: u32,
    pub velocity: u32,
}

impl Mass {
    pub fn build(cx: &Scope<MenuProps>, name: String, mass: u32, velocity: u32) -> Mass {
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
    pub fn get(cx: &Scope, id: u32) -> Mass {
        Mass {
            name: use_atom_ref(cx, MASS_NAMES)
                .read()
                .get(&id)
                .unwrap()
                .clone(),
            mass: use_atom_ref(cx, MASS_MASSES)
                .read()
                .get(&id)
                .unwrap()
                .clone(),
            velocity: use_atom_ref(cx, MASS_VELOCITIES)
                .read()
                .get(&id)
                .unwrap()
                .clone(),
        }
    }
}
