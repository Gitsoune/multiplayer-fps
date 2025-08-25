use crate::structs::{FpsCache, FpsText};
use bevy::prelude::*;

pub fn show_fps(
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
