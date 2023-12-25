mod player;
use player::PlayerPlugin;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const NUM_ENEMIES: usize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (
            spawn_camera,
            spawn_enemies,
        ))
        .run();
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let win = window_query.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let x_rand = rand::random::<f32>() * win.width();
        let y_rand = rand::random::<f32>() * win.height();
        
        commands.spawn(
            SpriteBundle{
                transform: Transform::from_xyz(x_rand, y_rand, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            }
        );
    }
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
