//! Game mechanics and content.

use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

pub mod assets;
pub mod audio;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        LookTransformPlugin,
        OrbitCameraPlugin::default(),
        InfiniteGridPlugin,
    ))
    .insert_resource(AmbientLight {
        brightness: 100.0,
        ..default()
    })
    .add_plugins((audio::plugin, assets::plugin, spawn::plugin));
}
