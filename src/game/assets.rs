use std::ops::Index;

use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    let next_state = Screen::Playing;
    #[cfg(not(feature = "dev"))]
    let next_state = Screen::Title;

    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(next_state)
            .load_collection::<CharactersAssets>()
            // .load_collection::<PlayerAssets>()
            .load_collection::<AudioAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct CharactersAssets {
    #[asset(path = "kenney-characters/Models/GLB format/character-male-a.glb")]
    pub male_a: Handle<Gltf>,
}

// #[derive(AssetCollection, Resource)]
// pub struct PlayerAssets {
//     #[asset(path = "kenney-characters/Models/GLB format/character-male-a.glb#Scene0")]
//     pub mesh: Handle<Scene>,
//
//     #[asset(path = "kenney-characters/Models/GLB format/character-male-a.glb#Animation2")]
//     pub animations: Handle<AnimationClip>,
// }

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // Sountracks
    #[asset(path = "audio/soundtracks/Monkeys Spinning Monkeys.ogg")]
    pub credits_soundtrack: Handle<AudioSource>,
    #[asset(path = "audio/soundtracks/Fluffing A Duck.ogg")]
    pub gameplay_soundtrack: Handle<AudioSource>,

    // SFX
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step1.ogg")]
    pub step1: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step2.ogg")]
    pub step2: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step3.ogg")]
    pub step3: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step4.ogg")]
    pub step4: Handle<AudioSource>,
}

impl Index<SoundtrackKey> for AudioAssets {
    type Output = Handle<AudioSource>;

    fn index(&self, index: SoundtrackKey) -> &Self::Output {
        match index {
            SoundtrackKey::Credits => &self.credits_soundtrack,
            SoundtrackKey::Gameplay => &self.gameplay_soundtrack,
        }
    }
}

impl Index<SfxKey> for AudioAssets {
    type Output = Handle<AudioSource>;

    fn index(&self, index: SfxKey) -> &Self::Output {
        match index {
            SfxKey::ButtonHover => &self.button_hover,
            SfxKey::ButtonPress => &self.button_press,
            SfxKey::Step1 => &self.step1,
            SfxKey::Step2 => &self.step2,
            SfxKey::Step3 => &self.step3,
            SfxKey::Step4 => &self.step4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}
