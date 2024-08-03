// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy3dtest::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
