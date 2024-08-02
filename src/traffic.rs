use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub enum TrafficDensity {
    Low,
    Medium,
    High,
}

#[derive(Resource)]
struct TrafficConfig {
    density: TrafficDensity,
    spawn_timer: Timer,
    update_timer: Timer,
}

#[derive(Component, Clone, Debug)]
struct AerialVehicle {
    name: String,
    altitude: f32,
    speed: f32,
    heading: f32,
    target: String,
}

impl AerialVehicle {
    /// Updates the AV positions at specified sample rate
    fn update_position(
        mut av_query: Query<(&Self, &mut Transform)>,
        timer: Res<Time>,
        mut config: ResMut<TrafficConfig>,
    ) {
        const KNOTS_TO_DIST: f32 = 1.0 / 30.0;

        // Tick timer
        config.update_timer.tick(timer.delta());

        if config.update_timer.finished() {
            // Update AV positions
            for (av, mut transform) in &mut av_query {
                let delta = Vec3::new(
                    (KNOTS_TO_DIST * av.speed)
                        * av.heading.cos()
                        * config.update_timer.elapsed_secs(),
                    (KNOTS_TO_DIST * av.speed)
                        * av.heading.sin()
                        * config.update_timer.elapsed_secs(),
                    0.0,
                );

                info!("Updating positing of {:?} by {:?}", &av, &delta);
                transform.translation += delta;
            }

            // Reset timer
            config.update_timer.reset();
        }
    }
}

pub struct TrafficPlugin {
    density: TrafficDensity,
}

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        let config = TrafficConfig {
            density: self.density,
            spawn_timer: self.create_spawn_timer(),
            update_timer: Timer::from_seconds(1.0, TimerMode::Once),
        };

        app.insert_resource(config).add_systems(
            Update,
            (Self::traffic_spawner, AerialVehicle::update_position),
        );
    }
}

impl TrafficPlugin {
    pub fn from_density(density: TrafficDensity) -> Self {
        Self { density }
    }

    fn create_spawn_timer(&self) -> Timer {
        match self.density {
            TrafficDensity::Low => Timer::new(Duration::from_secs_f32(15.0), TimerMode::Once),
            TrafficDensity::Medium => Timer::new(Duration::from_secs_f32(10.0), TimerMode::Once),
            TrafficDensity::High => Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once),
        }
    }

    fn spawn_av(
        av: AerialVehicle,
        x: f32,
        y: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(av.clone())
            .insert(SpatialBundle::from(Transform::from_xyz(x, y, 5.0)))
            .with_children(|parent| {
                // Text Label
                parent
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            format!(
                                "{} {}\n{} {} {}",
                                &av.name, &av.altitude, &av.speed, &av.heading, &av.target
                            ),
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(1.0, 1.0, 1.0),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)));

                // Icon
                parent
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                            Vec2::new(0.0, 5.0),
                            Vec2::new(-5.0, -6.0),
                            Vec2::new(5.0, -6.0),
                        ))),
                        material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            });
    }

    /// Spawn new traffic based on the density settings
    fn traffic_spawner(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        time: Res<Time>,
        mut config: ResMut<TrafficConfig>,
    ) {
        // Tick the timer
        config.spawn_timer.tick(time.delta());

        if config.spawn_timer.finished() {
            let mut rng = thread_rng();

            let av = AerialVehicle {
                name: format!(
                    "{}{}{}",
                    rng.gen_range(b'A'..=b'Z') as char,
                    rng.gen_range(b'A'..=b'Z') as char,
                    rng.gen_range(0o1..=999) as u16
                ),
                altitude: rng.gen_range(40000..=100000) as f32,
                speed: rng.gen_range(200..=500) as f32,
                heading: rng.gen_range(0..=360) as f32,
                target: "DXB".to_string(), // TODO: Get list of available airport
            };

            info!("Spawning new AV: {:?}", &av);

            // Reset timer
            config.spawn_timer.reset();

            // TODO: Setup proper spawn area
            let width_range = (-1.0 * crate::WINDOW_WIDTH / 2.5)..(crate::WINDOW_WIDTH / 2.5);
            let height_range = (-1.0 * crate::WINDOW_HEIGHT / 2.5)..(crate::WINDOW_HEIGHT / 2.5);

            let point = (
                rng.gen_range(width_range.clone()) as f32,
                rng.gen_range(height_range.clone()) as f32,
            );

            Self::spawn_av(
                av,
                point.0,
                point.1,
                &mut commands,
                &mut meshes,
                &mut materials,
            )
        }
    }
}
