use bevy::{log::LogPlugin, prelude::*};

use airport::AirportPlugin;
use traffic::TrafficPlugin;

use crate::airport::AirportLayouts;

mod airport;
mod traffic;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::INFO,
                ..Default::default()
            }),
            AirportPlugin::from_layout(AirportLayouts::Layout1),
            TrafficPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
