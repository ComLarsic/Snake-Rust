use crate::rendering;
use crate::food;
use crate as main;


use bevy::prelude::*;

/**
 * Snake
 */

/* --- Snake body --- */
pub struct Snakehead {
    pub direction : Direction
}

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<main::Position>);

/* --- Snake events --- */
pub struct GrowthEvent;

/* --- Snake statemachine --- */

// The current state of the snake
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeState {
    Input,
    Movement,
    Eating,
    Growth
}

// The direction for the snake
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl Direction {
    fn opposite(self) -> Self {
        return match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        };
    }
}

/* --- Spawning systems --- */

// Spawns the snake in the arena
pub fn spawn_snake(mut commands: Commands, materials: Res<rendering::Materials>, mut segments: ResMut<SnakeSegments>) {
    segments.0 = vec! [
        commands.spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Snakehead { direction: Direction::Up })
        .insert(main::Position {x: 3, y:3})
        .insert(main::Size::square(0.8))
        .id(),
        spawn_segment(
            commands, 
            &materials.head_material,
            main::Position { x:3, y:2 }
        ),
    ];
}

// Spawn a snake segment
pub fn spawn_segment(mut commands: Commands, material: &Handle<ColorMaterial>, position: main::Position) -> Entity {
    return commands.spawn_bundle(SpriteBundle {
        material: material.clone(),
        ..Default::default()
    })
    .insert(SnakeSegments)
    .insert(position)
    .insert(main::Size::square(0.65))
    .id();
}

/* --- Snake movmement --- */

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut Snakehead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = 
        if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

// Handles the snake movement
pub fn snake_movement(segments: ResMut<SnakeSegments>, mut heads: Query<(Entity, &Snakehead)>, mut positions: Query<&mut main::Position>, mut last_tail_position: ResMut<LastTailPosition>, mut game_over_writer: EventWriter<main::GameOverEvent>) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // Get segment positions
        let segment_position = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<main::Position>>();
        
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
        }

        if head_pos.x < 0 {
            head_pos.x = main::GameConf::ARENA_WIDTH as i32;
        } 
        if head_pos.x > main::GameConf::ARENA_WIDTH as i32 {
            head_pos.x = 0;
        }
        if head_pos.y < 0 {
            head_pos.y = main::GameConf::ARENA_HEIGHT as i32;
        } 
        if head_pos.y > main::GameConf::ARENA_WIDTH as i32 {
            head_pos.y = 0;
        }

        if segment_position.contains(&head_pos) {
            game_over_writer.send(main::GameOverEvent);
        }

        segment_position
        .iter()
        .zip(segments.0.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });

        last_tail_position.0 = Some(*segment_position.last().unwrap());
    }
}

/* --- Snake actions --- */

pub fn snake_eating(mut commands: Commands, mut growth_writer: EventWriter<GrowthEvent>, food_positions: Query<(Entity, &main::Position), With<food::Food>>, head_positions: Query<&main::Position, With<Snakehead>>) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(commands: Commands, last_tail_position: Res<LastTailPosition>, mut segment: ResMut<SnakeSegments>, mut growth_reader: EventReader<GrowthEvent>, materials: Res<rendering::Materials>) {
    if growth_reader.iter().next().is_some() {
        segment.0.push(spawn_segment(
            commands, 
            &materials.head_material,
            last_tail_position.0.unwrap()
        ))
    }
}