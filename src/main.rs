use std::time::Duration;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) //prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                // press the right arrow key to animate the right sprite
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                // press the left arrow key to animate the left sprite
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .run();
}

// This system runs when the user clicks the left arrow key or right arrow key
fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
    // we expect the Component of type S to be used as a marker Component by only a single entity
    let mut animation = query.single_mut();
    // we create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back tot he first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("characters/chars.png");

    // the sprite sheet has 72 sprites arranged in a grid, and they're all 14px x 18px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(14, 18), 18, 4, offset=UVec2::new(16,180)); 
    commands.spawn(SpriteBundle {
        texture: asset_server.load("characters/chars.png"),
        ..default()
    });
}
