mod galactica;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use galactica::GalacticaPlugin;

pub mod loading;
pub mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_plugins(GalacticaPlugin)
        .run();
}
