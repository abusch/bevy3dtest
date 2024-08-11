//! Spawn the main level by triggering other observers.

use avian3d::prelude::{Collider, RigidBody};
use bevy::{color::palettes, math::vec3, prelude::*};
use bevy_infinite_grid::InfiniteGridBundle;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use crate::{screen::Screen, MainCamera};

use super::{player::SpawnPlayer, scene::SpawnScene};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    main_camera: Query<Entity, With<MainCamera>>,
) {
    // Add directional light
    let transform = Transform::from_xyz(20.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        DirectionalLightBundle {
            transform,
            directional_light: DirectionalLight {
                illuminance: 4000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        Name::new("Sun"),
        StateScoped(Screen::Playing),
    ));

    // Setup camera controller
    let eye = vec3(2.0, 10.0, 8.0);
    let target = Vec3::ZERO;
    let camera = main_camera.single();
    commands.entity(camera).insert((OrbitCameraBundle::new(
        OrbitCameraController::default(),
        eye,
        target,
        Vec3::Y,
    ),));

    // Infinite grid plane
    commands.spawn((InfiniteGridBundle::default(), StateScoped(Screen::Playing)));

    // Floor
    commands.spawn((
        Name::new("Floor"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(128.0, 128.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        StateScoped(Screen::Playing),
    ));

    // box
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 2.0, 5.0).mesh()),
            material: materials.add(Color::from(palettes::basic::RED)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(5.0, 2.0, 5.0),
        StateScoped(Screen::Playing),
    ));

    commands.trigger(SpawnScene);
    commands.trigger(SpawnPlayer);
}
