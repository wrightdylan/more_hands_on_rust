use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use my_library::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    // #[default]
    MainMenu,
    #[default]
    Start,
    Player,
    Cpu,
    End,
    GameOver,
}

#[derive(Component)]
pub struct GameElement;

#[derive(Resource)]
struct GameAssets {
    atlas: Handle<TextureAtlasLayout>,
    image: Handle<Image>,
}

#[derive(Clone, Copy, Resource)]
struct Scores {
    player: usize,
    cpu:    usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)]
struct HandTimer(Timer);

#[derive(Resource)]
struct FinalScore(Scores);

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(GameElement);
    let texture_handle = asset_server.load("dice.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(52),
        6,
        1,
        None,
        None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(GameAssets { atlas: texture_atlas_handle, image: texture_handle });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(
    scores: Res<Scores>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(&format!("Player: {}", scores.player));
        ui.label(&format!("CPU: {}", scores.cpu));
    });
}

fn spawn_die(
    hand_query: &Query<(Entity, &TextureAtlas), With<HandDie>>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let rolled_die = hand_query.iter().count() as f32 * 52.0;
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    ..default()
                },
                texture: assets.image.clone(),
                transform: Transform::from_xyz(rolled_die - 400.0, 60.0, 1.0),
                ..default()
            },
            TextureAtlas {
                layout: assets.atlas.clone(),
                index: new_roll - 1,
            },
        ))
        .insert(HandDie)
        .insert(GameElement);
}

fn clear_die(
    hand_query: &Query<(Entity, &TextureAtlas), With<HandDie>>,
    commands: &mut Commands,
) {
    hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn player(
    hand_query: Query<(Entity, &TextureAtlas), With<HandDie>>,
    mut commands: Commands,
    rng: Res<RandomNumberGenerator>,
    assets: Res<GameAssets>,
    mut scores: ResMut<Scores>,
    mut state: ResMut<NextState<GamePhase>>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Play Options").show(egui_context.ctx_mut(), |ui| {
        let hand_score: usize = hand_query.iter().map(|(_, ts)| ts.index + 1).sum();
        ui.label(&format!("Score for this hand: {hand_score}"));

        if ui.button("Roll Dice").clicked() {
            let new_roll = rng.range(1..=6);
            if new_roll == 1 {
                // End turn!
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Cpu);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll,
                    Color::WHITE,
                );
            }
        }
        if ui.button("Pass - Keep Hand Score").clicked() {
            let hand_total: usize = hand_query.iter().map(|(_, ts)| ts.index + 1).sum();
            scores.player += hand_total;
            clear_die(&hand_query, &mut commands);
            state.set(GamePhase::Cpu);
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn cpu(
    hand_query: Query<(Entity, &TextureAtlas), With<HandDie>>,
    mut state: ResMut<NextState<GamePhase>>,
    mut scores: ResMut<Scores>,
    rng: Res<RandomNumberGenerator>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut timer: ResMut<HandTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let hand_total: usize = hand_query.iter().map(|(_, ts)| ts.index + 1).sum();
        if hand_total < 20 && scores.cpu + hand_total < 100 {
            let new_roll = rng.range(1..=6);
            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Player);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll,
                    Color::LinearRgba(LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }),
                );
            }
        } else {
            scores.cpu += hand_total;
            state.set(GamePhase::Player);
            hand_query
                .iter()
                .for_each(|(entity, _)| commands.entity(entity).despawn());
        }
    }
}

fn start_game(mut state: ResMut<NextState<GamePhase>>) {
    state.set(GamePhase::Player);
}

fn end_game(
    mut state: ResMut<NextState<GamePhase>>,
    scores: Res<Scores>,
    mut commands: Commands,
) {
    commands.insert_resource(FinalScore(*scores));
    state.set(GamePhase::GameOver);
}

fn check_game_over(
    mut state: ResMut<NextState<GamePhase>>,
    scores: Res<Scores>,
) {
    if scores.cpu >= 100 || scores.player >= 100 {
        state.set(GamePhase::End);
    }
}

fn display_final_score(
    scores: Res<FinalScore>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(&format!("Player: {}", scores.0.player));
        ui.label(&format!("CPU: {}", scores.0.cpu));
        if scores.0.player < scores.0.cpu {
            ui.label("CPU is the winner!")
        } else {
            ui.label("Player is the winner!")
        }
    });
}

fn main() {
    let mut app = App::new();

    add_phase!(app, GamePhase, GamePhase::Start,
        start => [ setup ], run => [ start_game ], exit => [ ]
    );

    add_phase!(app, GamePhase, GamePhase::Player,
        start => [ ], run => [ player, check_game_over, display_score ], exit => [ ]
    );

    add_phase!(app, GamePhase, GamePhase::Cpu,
        start => [ ], run => [ cpu, check_game_over, display_score ], exit => [ ]
    );

    add_phase!(app, GamePhase, GamePhase::End,
        start => [ ], run => [ end_game ], exit => [ cleanup::<GameElement> ]
    );

    add_phase!(app, GamePhase, GamePhase::GameOver,
        start => [ ], run => [ display_final_score ], exit => [ ]
    );

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pig - Bevy Edition".to_string(),
            resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
            ..default()
        }),
        ..default()
        }))
        .add_plugins(GameStatePlugin::new(
            GamePhase::MainMenu,
            GamePhase::Start,
            GamePhase::GameOver,
        ))
        .add_plugins(EguiPlugin)
        .add_plugins(RandomPlugin)
        .run();
}
