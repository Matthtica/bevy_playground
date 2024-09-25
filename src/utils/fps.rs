use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};

pub fn say_hi() {
    println!("Hello, world!");
}

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps)
            .add_systems(Update, update_fps);
    }
}

fn setup_fps(mut commands: Commands) {
    let layout = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };
    commands.spawn(layout)
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("FPS: 0.0", TextStyle::default()),
            ));
        });
}

pub fn update_fps(mut query: Query<&mut Text>, diagnostics: Res<DiagnosticsStore>) {
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).map(|x| x.average().unwrap_or_default()).unwrap_or_default();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.2} ", fps);
    }
}
