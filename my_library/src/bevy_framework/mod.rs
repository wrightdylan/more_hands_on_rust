use bevy::{prelude::*, state::state::FreelyMutableState};

pub struct GameStatePlugin<S> {
    menu_state: S,
    game_start_state: S,
    game_end_state: S,
}

impl <S> GameStatePlugin<S>
{
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: S, game_start_state: S, game_end_state: S) -> Self {
        Self { menu_state, game_start_state, game_end_state }
    }
}

impl<S: FreelyMutableState + FromWorld> Plugin for GameStatePlugin<S>
{
    fn build(&self, app: &mut App) {
        app.init_state::<S>();
    }
}