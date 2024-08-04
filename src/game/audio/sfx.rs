use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::{AudioAssets, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(trigger: Trigger<PlaySfx>, mut commands: Commands, audio_assets: Res<AudioAssets>) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        PlaySfx::RandomStep => random_step(),
    };
    commands.spawn(AudioSourceBundle {
        source: audio_assets[sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    #[allow(dead_code)]
    RandomStep,
}

fn random_step() -> SfxKey {
    [SfxKey::Step1, SfxKey::Step2, SfxKey::Step3, SfxKey::Step4]
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap()
}
