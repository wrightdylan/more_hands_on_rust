use super::{MenuAssets, MenuResource};
use bevy::{app::AppExit, prelude::*, state::state::FreelyMutableState};

#[derive(Component)]
pub(crate) struct MenuElement;

pub(crate)fn setup<S>(
    state: Res<State<S>>,
    mut commands: Commands,
    menu_assets: Res<MenuAssets>,
    menu_resource: Res<MenuResource<S>>,
) where
    S: FreelyMutableState + FromWorld,
{
    let current_state = state.get();
    let menu_graphic = match current_state {
        current_state if menu_resource.menu_state == *current_state => menu_assets.main_menu.clone(),
        current_state if menu_resource.game_end_state == *current_state => menu_assets.game_over.clone(),
        _ => panic!("Unknown menu state"),
    };

    commands
        .spawn(Camera2dBundle::default())
        .insert(MenuElement);
    commands
        .spawn(SpriteBundle {
            texture: menu_graphic,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(MenuElement);
}

pub(crate) fn run<S>(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    current_state: Res<State<S>>,
    mut state: ResMut<NextState<S>>,
    menu_state: Res<MenuResource<S>>,
) where
    S: FreelyMutableState + FromWorld,
{
    let current_state = current_state.get().clone();
    if current_state == menu_state.menu_state {
        if keyboard.just_pressed(KeyCode::KeyP) {
            state.set(menu_state.game_start_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    } else if current_state == menu_state.game_end_state {
        if keyboard.just_pressed(KeyCode::KeyM) {
            state.set(menu_state.game_end_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}