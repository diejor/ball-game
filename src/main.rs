use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const SPEED: f32 = 500.0; // pixels per second
pub const PLAYER_SIZE: f32 = 64.0; // pixels
pub const NUM_ENEMIES: usize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_enemies,
        ))
        .add_systems(Update, (
            confine_player,
            move_player,
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
    let width_win: f32 = window.width();
    let height_win: f32 = window.height();

    let path_to_sprite = String::from("sprites/ball_blue_large.png");
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(width_win/2.0, height_win/2.0, 0.0),
            texture: asset_server.load(&path_to_sprite),
            ..default()
        },
        Player {}
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let win = window_query.get_single().unwrap();
    alexander.

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

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    user_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = player_query.single_mut();

    let mut delta_velocity = Vec3::ZERO;

    for key in user_input.get_pressed() {
        match key {
            KeyCode::W | KeyCode::Up => delta_velocity += Vec3::new(0.0, 1.0, 0.0),
            KeyCode::S | KeyCode::Down => delta_velocity += Vec3::new(0.0, -1.0, 0.0),
            KeyCode::A | KeyCode::Left => delta_velocity += Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::D | KeyCode::Right => delta_velocity += Vec3::new(1.0, 0.0, 0.0),
            _ => (),
        };
    }

    if delta_velocity.length() > 1.0 {
        delta_velocity = delta_velocity.normalize();
    }

    transform.translation += delta_velocity * time.delta_seconds() * SPEED;
}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.single();
    let width_win = window.width();
    let height_win = window.height();

    let mut transform = player_query.single_mut();

    // this vector is not nessesaryly a position vector, rather a vector that contains information about the size of the player
    let player_size = Vec3::new(PLAYER_SIZE/2.0, PLAYER_SIZE/2.0, 0.0);

    // note this points to the right top
    let max_transform = Vec3::new(width_win, height_win, 0.0) - player_size;

    // note this points to the left bottom
    let min_transform = Vec3::new(0.0, 0.0, 0.0) + player_size;

    // confine player
    transform.translation = transform.translation.max(min_transform).min(max_transform);
}


