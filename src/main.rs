use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{WindowResolution, WindowTheme},
};

use airport::AirportPlugin;
use traffic::TrafficPlugin;

use crate::airport::AirportLayouts;

mod airport;
mod traffic;

const WINDOW_WIDTH: f32 = 1500.0;
const WINDOW_HEIGHT: f32 = 900.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                            .with_scale_factor_override(1.0),
                        resizable: false,
                        title: "Mayday".to_string(),
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
            AirportPlugin::from_layout(AirportLayouts::ThreeRandom),
            TrafficPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
