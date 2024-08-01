use bevy::prelude::*;

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, say_hi);
    }
}

fn say_hi() {
    info!("Hoi");
}
