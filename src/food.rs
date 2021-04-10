/**
 * Food
 */
use crate as main;
use crate::rendering;

use bevy::prelude::*;
use rand::prelude::random;

pub struct Food;

pub fn food_spawner(mut commands: Commands, materials: Res<rendering::Materials>) {
    
    let x = (random::<f32>() * main::GameConf::ARENA_WIDTH as f32) as i32;
    let y = (random::<f32>() * main::GameConf::ARENA_HEIGHT as f32) as i32;

    commands.spawn_bundle(SpriteBundle {
        material: materials.food_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    })
    .insert(Food)
    .insert(main::Position {
        x: x,
        y: y
    })
    .insert(main::Size::square(0.8));
}