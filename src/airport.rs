use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub enum AirportLayouts {
    ThreeRandom,
}

#[derive(Resource)]
struct AirportConfig {
    layout: AirportLayouts,
}

#[derive(Component)]
struct Airport {
    name: String,
}

pub struct AirportPlugin {
    layout: AirportLayouts,
}

impl Plugin for AirportPlugin {
    fn build(&self, app: &mut App) {
        let config = AirportConfig {
            layout: self.layout,
        };
        app.insert_resource(config)
            .add_systems(Startup, Self::setup_layout);
    }
}

impl AirportPlugin {
    pub fn from_layout(layout: AirportLayouts) -> Self {
        Self { layout }
    }
}

impl AirportPlugin {
    fn spawn_airport(
        x: f32,
        y: f32,
        name: impl Into<String>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let name = name.into();

        commands
            .spawn(Airport { name: name.clone() })
            .insert(SpatialBundle::from(Transform::from_xyz(x, y, 0.0)))
            .with_children(|parent| {
                // Text Label
                parent
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            name,
                            TextStyle {
                                font_size: 16.0,
                                color: Color::srgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(0.0, -25.0, 2.0)));

                // Landing Radius
                parent.spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(50.0))),
                    material: materials.add(Color::srgb(1.0, 204.0 / 255.0, 203.0 / 255.0)),
                    ..default()
                });

                // Center
                parent
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 25.0))),
                        material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));
            });
    }

    /// Get relative distances from a new airport to the current ones
    fn get_relative_distances(locs: &[(f32, f32)], new: (f32, f32)) -> Vec<f32> {
        info!("Checking distance between {:?} and {:?}", &new, locs);

        locs.iter()
            .map(|x| ((new.0 - x.0).powf(2.0) + (new.1 - x.1).powf(2.0)).sqrt())
            .collect()
    }

    /// Check to see if any distance is within threshold
    fn check_distance_thresh(distances: Vec<f32>, thresh: f32) -> bool {
        info!("Checking distance tresh {:?} and {:?}", &distances, &thresh);

        distances.iter().any(|x| x.abs() <= thresh)
    }

    fn create_three_random(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        info!("AirportLayouts::ThreeRandom Creating 3 airports with random start locations");

        const DISTANCE_THRESH: f32 = 200.0;

        let mut rng = thread_rng();

        let width_range = (-1.0 * crate::WINDOW_WIDTH / 2.5)..(crate::WINDOW_WIDTH / 2.5);
        let height_range = (-1.0 * crate::WINDOW_HEIGHT / 2.5)..(crate::WINDOW_HEIGHT / 2.5);

        let mut locs: Vec<(f32, f32)> = Vec::new();
        for _ in 0..3 {
            let mut point = (
                rng.gen_range(width_range.clone()) as f32,
                rng.gen_range(height_range.clone()) as f32,
            );

            // Ensure there are no duplicates
            while Self::check_distance_thresh(
                Self::get_relative_distances(&locs, point),
                DISTANCE_THRESH,
            ) {
                point = (
                    rng.gen_range(width_range.clone()) as f32,
                    rng.gen_range(height_range.clone()) as f32,
                );
            }

            info!(
                "AirportLayouts::ThreeRandom Created airport at {:?}",
                &point
            );
            locs.push(point);
        }

        Self::spawn_airport(locs[0].0, locs[0].1, "YYZ", commands, meshes, materials);
        Self::spawn_airport(locs[1].0, locs[1].1, "LAX", commands, meshes, materials);
        Self::spawn_airport(locs[2].0, locs[2].1, "DXB", commands, meshes, materials);
    }

    fn setup_layout(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        config: Res<AirportConfig>,
    ) {
        match config.layout {
            AirportLayouts::ThreeRandom => {
                AirportPlugin::create_three_random(&mut commands, &mut meshes, &mut materials);
            }
        }
    }
}
