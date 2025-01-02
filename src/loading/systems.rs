use bevy::prelude::*;

use crate::galactica::{GameEvent, GameState};

use super::components::LoadingScreen;

pub fn setup(mut cmd: Commands) {
    // Spawns the UI that will show the user prompts.
    let text_style = TextFont {
        font_size: 16.0,
        ..default()
    };
    cmd.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Node {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        BackgroundColor(Color::NONE),
        LoadingScreen,
    ))
    .with_child((Text::new("Loading..."), text_style));
}

pub fn on_exit(mut cmd: Commands, loading_query: Query<Entity, With<LoadingScreen>>) {
    for entity in loading_query.iter() {
        cmd.entity(entity).despawn();
    }
}

#[derive(Resource)]
pub struct LoadingTimer {
    pub timer: Timer,
}

impl Default for LoadingTimer {
    fn default() -> LoadingTimer {
        LoadingTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub fn tick_loading_timer(time: Res<Time>, mut loading_timer: ResMut<LoadingTimer>) {
    loading_timer.timer.tick(time.delta());
}

pub fn loading_over_time(
    loading_timer: ResMut<LoadingTimer>,
    mut game_event: EventWriter<GameEvent>,
) {
    if loading_timer.timer.finished() {
        // once done loading, go to menu.
        game_event.send(GameEvent {
            game_state: GameState::InMenu,
        });
    }
}
