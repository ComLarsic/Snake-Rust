mod rendering;
mod snake;
mod food;

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::render::pass::ClearColor;

/**
 * GameLogic
 */
struct GameConf;

impl GameConf {
    const ARENA_WIDTH: u32 = 25;
    const ARENA_HEIGHT: u32 = 25;
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32,
    y: i32
}

pub struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn square(x: f32) -> Self {
        return Self {
            width: x,
            height: x
        }
    }
}

pub struct GameOverEvent;

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        // Window config
        app
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 1280.0,
            height: 1280.0,
            ..Default::default()
        })
        // Add systems
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(snake::spawn_snake.system()))
        .add_event::<snake::GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_system(
            snake::snake_movement_input
            .system()
            .label(snake::SnakeState::Input)
            .before(snake::SnakeState::Movement),
        )
        .add_system(
            snake::snake_growth
            .system()
            .label(snake::SnakeState::Growth)
            .after(snake::SnakeState::Eating),
        )
        .add_system(
            game_over.system()
            .after(snake::SnakeState::Movement)
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(5.0))
            .with_system(food::food_spawner.system())
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.06))
            .with_system(snake::snake_movement.system().label(snake::SnakeState::Movement))
            .with_system(snake::snake_eating.system().label(snake::SnakeState::Eating).after(snake::SnakeState::Movement)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
            .with_system(size_scaling.system())
            .with_system(position_translation.system())
        )
        .insert_resource(snake::SnakeSegments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_plugins(DefaultPlugins);
    }
}

// Setup the game
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(rendering::Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        food_material: materials.add(Color::rgb(0.7, 0.0, 0.0).into())
    });
}

// Scale the tiles to the size
pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        let width = sprite_size.width / GameConf::ARENA_WIDTH as f32 * window.width() as f32;
        let height = sprite_size.height / GameConf::ARENA_HEIGHT as f32 * window.height() as f32;

        sprite.size = Vec2::new(width, height);
    }
}

// Translate the tiles position
pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        return pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.);
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), GameConf::ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), GameConf::ARENA_HEIGHT as f32),
            0.0
        );
    }
}

// Handle GameOver events
fn game_over(mut commands: Commands, mut reader: EventReader<GameOverEvent>, materials: Res<rendering::Materials>, segment_res: ResMut<snake::SnakeSegments>, food: Query<Entity, With<food::Food>>, segments: Query<Entity, With<snake::SnakeSegments>>, heads : Query<Entity, With<snake::Snakehead>>, sprites : Query<Entity, With<Handle<ColorMaterial>>>) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter().chain(heads.iter().chain(sprites.iter()))) {
            commands.entity(ent).despawn();
        }

        snake::spawn_snake(commands, materials, segment_res);
    }
}

fn main() {
    // Build app
    App::build()
    .add_plugin(Game)
    .run();
}
