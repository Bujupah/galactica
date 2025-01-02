mod components;
mod systems;

use crate::galactica::GameState;
use bevy::prelude::*;

use systems::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTimer>()
            .add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(OnExit(GameState::Loading), on_exit)
            .add_systems(
                Update,
                (tick_loading_timer, loading_over_time).run_if(in_state(GameState::Loading)),
            ); // Systems
    }
}
