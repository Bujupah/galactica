pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

use crate::galactica::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(OnExit(GameState::InGame), despawn_player)
            .add_systems(Update, player_mouvement.run_if(in_state(GameState::InGame)));
    }
}
