use bevy::{
    prelude::*,
    sprite:: Wireframe2dPlugin,
    window::{PresentMode, Window, WindowTheme, WindowPlugin},
};

use bevy_playground::{
    utils::FpsPlugin,
    demo::MovingCirclePlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy fun".into(),
            name: Some("bevy_fun_app".into()),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Light),
            ..default()
        }),
        ..default()
    }), Wireframe2dPlugin))
        .add_plugins(FpsPlugin)
        .add_plugins(MovingCirclePlugin)
        .run();
}
