use crate::player_movement;
use crate::prompts::{prompt_server_addr, prompt_username};
use crate::structs::{FpsCache, FpsText, Player};
use bevy::prelude::*;
use bevy::sprite::Sprite;

pub fn run() {
    let server_addr = prompt_server_addr();
    let username = prompt_username();
    println!(
        "Starting Bevy UI Client... (server: {}, username: {})",
        server_addr, username
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FpsCache { last: -1 })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (crate::show_fps::show_fps, player_movement::player_movement),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 255.0, 0.0),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Player,
    ));
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font: default(),
                font_size: 42.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font: default(),
                font_size: 33.0,
                ..default()
            },
            FpsText,
        ));
}
