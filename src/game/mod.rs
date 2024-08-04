//! Game mechanics and content.

use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod animation;
pub mod assets;
pub mod audio;
mod movement;
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
    .add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
    ));
}
