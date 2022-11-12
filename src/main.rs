#![windows_subsystem = "windows"]

use lagrange_points_simulator::app;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            w.with_resizable(true)
                .with_inner_size(dioxus::desktop::wry::application::dpi::LogicalSize::new(
                    1280.0, 720.0,
                ))
                .with_min_inner_size(dioxus::desktop::wry::application::dpi::LogicalSize::new(
                    1280.0, 720.0,
                ))
                .with_title("Lagrange Point Simulator")
        })
    });
}
