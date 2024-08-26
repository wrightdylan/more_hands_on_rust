use bevy::{prelude::*, state::state::FreelyMutableState};

mod game_menus;

pub struct GameStatePlugin<S> {
    menu_state: S,
    game_start_state: S,
    game_end_state: S,
}

impl<S> GameStatePlugin<S>
where
    S: FreelyMutableState + FromWorld
{
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: S, game_start_state: S, game_end_state: S) -> Self {
        Self { menu_state, game_start_state, game_end_state }
    }
}

impl<S> Plugin for GameStatePlugin<S>
where
    S: FreelyMutableState + FromWorld + Copy,
{
    fn build(&self, app: &mut App) {
        app.init_state::<S>();
        app.add_systems(Startup, setup_menus);
        let start = MenuResource {
            menu_state: self.menu_state,
            game_start_state: self.game_start_state,
            game_end_state: self.game_end_state,
        };
        app.insert_resource(start);

        app.add_systems(OnEnter(self.menu_state), game_menus::setup::<S>);
        app.add_systems(Update, game_menus::run::<S>
            .run_if(in_state(self.menu_state)));
        app.add_systems(OnExit(self.menu_state), cleanup::<game_menus::MenuElement>);

        app.add_systems(OnEnter(self.game_end_state), game_menus::setup::<S>);
        app.add_systems(Update, game_menus::run::<S>
            .run_if(in_state(self.game_end_state)));
        app.add_systems(OnExit(self.game_end_state), cleanup::<game_menus::MenuElement>);
    }
}

#[derive(Resource)]
pub(crate) struct MenuAssets {
    pub(crate) main_menu: Handle<Image>,
    pub(crate) game_over: Handle<Image>,
}

#[derive(Resource)]
pub(crate) struct MenuResource<S> {
    pub(crate) menu_state: S,
    pub(crate) game_start_state: S,
    pub(crate) game_end_state: S,
}

fn setup_menus(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = MenuAssets {
        main_menu: asset_server.load("main_menu.png"),
        game_over: asset_server.load("game_over.png"),
    };
    commands.insert_resource(assets);
}

pub fn cleanup<S>(
    query: Query<Entity, With<S>>,
    mut commands: Commands,
)
where S: Component
{
    query.iter().for_each(|entity| commands.entity(entity).despawn())
}

#[macro_export]
macro_rules! add_phase {
    (
        $app:expr, $type:ty, $phase:expr,
        start => [ $($start:expr),* ],
        run   => [ $($run:expr),* ],
        exit  => [ $($exit:expr),* ]
    ) => {
        $($app.add_systems(
            bevy::prelude::OnEnter::<$type>($phase),
            $start
        );)*
        $($app.add_systems(
            bevy::prelude::Update, $run.run_if(in_state($phase))
        );)*
        $($app.add_systems(
            bevy::prelude::OnExit::<$type>($phase),
            $exit
        );)*
    };
}