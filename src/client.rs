use bevy::prelude::*;
use std::io::{self, Write};

// Minimal Bevy client with an FPS overlay (works with Bevy 0.16.1).

#[derive(Component)]
struct FpsText;

// Small resource that stores the last-displayed integer FPS to avoid
// reformatting and updating the UI every frame when the value hasn't changed.
#[derive(Resource)]
struct FpsCache {
    last: i32,
}

pub fn run() {
    // Prompt (optional)
    print!("Enter IP Address ( 127.0.0.1:8080 ) : ");
    io::stdout().flush().ok();
    let mut _ip = String::new();
    let _ = io::stdin().read_line(&mut _ip);

    print!("Enter Name : ");
    io::stdout().flush().ok();
    let mut _username = String::new();
    let _ = io::stdin().read_line(&mut _username);

    println!("Starting Bevy UI Client...");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FpsCache { last: -1 })
        .add_systems(Startup, setup)
        .add_systems(Update, show_fps)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera (example uses the unit `Camera2d` marker)
    commands.spawn(Camera2d);

    // Spawn a parent Text with one section "FPS: " and a child TextSpan which
    // we will update each frame. This mirrors the Bevy example structure.
    commands
        .spawn((
            // Parent text with the label
            Text::new("FPS: "),
            // Styling for the parent text
            TextFont {
                font: default(),
                font_size: 42.0,
                ..default()
            },
        ))
        .with_child((
            // Child span that will contain the numeric FPS value.
            TextSpan::default(),
            TextFont {
                font: default(),
                font_size: 33.0,
                ..default()
            },
            // Marker so we can query the span later
            FpsText,
        ));
}

fn show_fps(
    time: Res<Time>,
    mut cache: ResMut<FpsCache>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    // Compute integer FPS and only update UI when it changes.
    let fps_i = (1.0 / dt).round() as i32;
    if fps_i == cache.last {
        return;
    }
    cache.last = fps_i;

    let display = fps_i.to_string();
    for mut span in &mut query {
        **span = display.clone();
    }
}
