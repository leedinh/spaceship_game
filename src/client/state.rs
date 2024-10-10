use bevy::{prelude::*, scene::ron::de};

#[derive(Debug, Default, States, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
    Paused,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, game_state_input_events)
            .add_systems(First, start_game);
    }
}

pub fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (),
        }
    }
}

pub fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info!("Current state: {:?}", state.get());
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Loading => next_state.set(GameState::InGame),
            GameState::GameOver => next_state.set(GameState::Loading),
            _ => (),
        }
    }
    info!("Current state: {:?}", state.get());
}
