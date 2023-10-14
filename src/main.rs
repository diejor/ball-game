use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player
        ))
        .add_systems(Update, (
            move_player
        ))
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = windows_query.get_single().unwrap();
    let width_win = window.width();
    let height_win = window.height();
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(width_win/2.0, height_win/2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {}
    )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = windows_query.get_single().unwrap();
    let width_win = window.width();
    let height_win = window.height();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(width_win/2.0, height_win/2.0, 0.0),
            ..default()
       }
    );
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    user_input: Res<Input<KeyCode>>
) {

    let transform: &mut Transform = &mut player_query.single_mut();

    for key in user_input.get_pressed() {
        let delta_velocity = match key {
            KeyCode::W | KeyCode::Up => Vec3::new(0.0, 1.0, 0.0),
            KeyCode::S | KeyCode::Down => Vec3::new(0.0, -1.0, 0.0),
            KeyCode::A | KeyCode::Left => Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::D | KeyCode::Right => Vec3::new(1.0, 0.0, 0.0),
            _ => Vec3::ZERO,
        };

        transform.translation += delta_velocity;
    }    
}