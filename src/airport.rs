use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Copy, Clone)]
pub enum AirportLayouts {
    Layout1,
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
            .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
            .insert(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 25.0))),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            name,
                            TextStyle {
                                font_size: 15.0,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(0.0, -25.0, 0.0)));
            });
    }

    fn setup_layout(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        config: Res<AirportConfig>,
    ) {
        match config.layout {
            AirportLayouts::Layout1 => {
                Self::spawn_airport(0.0, 0.0, "YYZ", &mut commands, &mut meshes, &mut materials);
            }
        }
    }
}
