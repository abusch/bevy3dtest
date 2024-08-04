//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, math::vec3, prelude::*};
use bevy_infinite_grid::InfiniteGridBundle;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use super::Screen;
use crate::{
    game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel},
    MainCamera,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands, main_camera: Query<Entity, With<MainCamera>>) {
    // Add directional light
    let transform = Transform::from_xyz(20.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        DirectionalLightBundle {
            transform,
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    // Setup camera controller
    let eye = vec3(10.0, 10.0, 10.0);
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

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    commands.trigger(SpawnLevel);
}

fn exit_playing(mut commands: Commands, main_camera: Query<Entity, With<MainCamera>>) {
    // Remove orbit controller
    let camera = main_camera.single();
    commands.entity(camera).remove::<OrbitCameraBundle>();

    // We could use [`StateScoped`] on the sound playing entities instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
