use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Menu(cx: Scope) -> Element {
    let cog_icon = include_str!("../assets/icons/cog.svg");
    let show_menu = use_state(&cx, || "hide");
    let value_slider1 = use_state(&cx, || 10);
    let value_slider2 = use_state(&cx, || 10);
    let value_slider3 = use_state(&cx, || 10);

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

                h1 {
                    "Parameters :"
                }

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
                    div {
                        class: "row",
                        label {
                            "for":"slider2",
                            "Mass of 1st object"
                        }
                        div {
                            class: "column",
                            input {
                                id: "slider2",
                                class: "slider",
                                "type": "range",
                                min: "1",
                                max: "100",
                                value: "{value_slider2}",
                                oninput: move |evt| {
                                    value_slider2.set(evt.value.clone().parse().unwrap());
                                },
                            }
                        }
                        p {
                            class: "value",
                            dangerous_inner_html: "{value_slider2} kg",
                        }
                    }

                    div {
                        class: "row",
                        label {
                            "for":"slider3",
                            "Mass of 2nd object"
                        }
                        div {
                            class: "column",
                            input {
                                id: "slider3",
                                class: "slider",
                                "type": "range",
                                min: "1",
                                max: "100",
                                value: "{value_slider3}",
                                oninput: move |evt| {
                                    value_slider3.set(evt.value.clone().parse().unwrap());
                                }
                            }
                        }
                        p {
                            class: "value",
                            dangerous_inner_html: "{value_slider3} kg",
                        }
                    }
                }

            }
        }
    ))
}
