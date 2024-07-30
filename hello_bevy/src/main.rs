use bevy::prelude::*;

#[derive(Component)]
struct Dragon;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("dragon.png"),
            ..Default::default()
        })
        .insert(Dragon);
}

fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dragon_query: Query<&mut Transform, With<Dragon>>,
) {
    let delta = if keyboard.pressed(KeyCode::ArrowLeft) {
        Vec2::new(-1.0, 0.0)
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        Vec2::new(1.0, 0.0)
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        Vec2::new(0.0, -1.0)
    } else if keyboard.pressed(KeyCode::ArrowUp) {
        Vec2::new(0.0, 1.0)
    } else {
        Vec2::ZERO
    };

    dragon_query.iter_mut().for_each(|mut transform| {
        transform.translation += delta.extend(0.0);
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}
