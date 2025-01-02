use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::loading::LoadingPlugin;

pub struct GalacticaPlugin;
impl Plugin for GalacticaPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_state::<GameState>()
            .add_event::<GameEvent>()
            .add_audio_channel::<GameBackgroundAudio>()
            .add_plugins(LoadingPlugin)
            .add_systems(Startup, background_audio)
            .add_systems(Update, game_event_handler);
    }
}

#[allow(dead_code)]
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    InMenu,
    InGame,
}

#[derive(Event)]
pub struct GameEvent {
    pub game_state: GameState,
}

#[derive(Resource)]
struct GameBackgroundAudio;

fn game_event_handler(
    mut game_event_listener: EventReader<GameEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in game_event_listener.read() {
        game_state.set(event.game_state);
        info!("Navigated to: {:?}", event.game_state);
    }
}

fn background_audio(
    asset_server: Res<AssetServer>,
    background: Res<AudioChannel<GameBackgroundAudio>>,
) {
    background
        .play(asset_server.load("galactica/sounds/outerspace_sound.mp3"))
        .with_volume(0.1)
        .looped();
}
