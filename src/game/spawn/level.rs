//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

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
) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(128.0, 128.0)),
        material: materials.add(Color::WHITE),
        ..default()
    });
    commands.trigger(SpawnScene);
    commands.trigger(SpawnPlayer);
}
