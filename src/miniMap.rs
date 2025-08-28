use crate::maze::MAP1;
use bevy::prelude::*;
use bevy::sprite::Sprite;

    // === Minimap ===
pub fn mini_map(mut commands : Commands) {
    let tile_size = 8.0; // each cell = 10x10 pixels
    let offset_x = -590.0; // shift minimap left
    let offset_y = 250.0;  // shift minimap up

    for (row, line) in MAP1.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let color = if cell == 1 {
                Color::BLACK
            } else {
                Color::WHITE
            };
            commands.spawn(Sprite {
                color,
                custom_size: Some(Vec2::splat(tile_size)),
                ..default()
            })
            .insert(Transform::from_xyz(
                offset_x + col as f32 * tile_size,
                offset_y - row as f32 * tile_size,
                -55.0, 
            ));
        }
    }
}