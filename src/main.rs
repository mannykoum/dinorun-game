use bevy::prelude::*;
use bevy::{input::keyboard::*, transform};
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, RepeatStrategy,
};

const PLAYER_SPRITE: &str = "player.png";
const BACKGROUND: &str = "background-sunset/sky.png";
const FLOOR: &str = "background-sunset/ground.png";
const MOUNTAINS: &str = "background-sunset/mountains.png";
const FOREGROUND: &str = "background-sunset/foreground.png";

// spritesheet animation indices
const WALK_ANIMATION: (usize, usize) = (0, 11);
const RUN_ANIMATION: (usize, usize) = (12, 19);
const JUMP_ANIMATION: (usize, usize) = (20, 24);
const FALL_ANIMATION: (usize, usize) = (25, 29);

const GROUND_Y: f32 = -64.0;
const WALK_SPEED: f32 = 1.0;
const RUN_SPEED: f32 = 1.5;
const GRAVITY: f32 = 9.8;
// Jumping parameters
const JUMP_HEIGHT: f32 = 122.0;
const JUMP_SPEED: f32 = 9.8 * 1.5;

const ANIM_TIME: f32 = 0.1;

// Player state
#[derive(Debug, PartialEq, Eq)]
enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Running,
    Falling,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
// Res and ResMut provide read and write access to resources respectively

// Floor component
#[derive(Component)]
struct Floor;

// Player component
#[derive(Component)]
struct Player {
    on_ground: bool,
    state: PlayerState,
}

// Animation indices
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
// system to change animation indices based on player state
fn change_animation(mut player_query: Query<(&Player, &mut TextureAtlas, &mut AnimationIndices)>) {
    let (player, mut atlas, mut indices) = player_query.single_mut();
    let pr_first = indices.first;
    let pr_last = indices.last;
    match player.state {
        PlayerState::Walking => {
            indices.first = WALK_ANIMATION.0;
            indices.last = WALK_ANIMATION.1;
            if atlas.index < indices.first || atlas.index > indices.last {
                // map to the appropriate index of the walk animation
                let prev_length = pr_last - pr_first;
                let curr_length = indices.last - indices.first;
                let index = atlas.index - pr_first;
                let percentage = index as f32 / prev_length as f32;
                atlas.index = (percentage * curr_length as f32).round() as usize + indices.first;
            }
        }
        PlayerState::Running => {
            indices.first = RUN_ANIMATION.0;
            indices.last = RUN_ANIMATION.1;
            if atlas.index < indices.first || atlas.index > indices.last {
                // map to the appropriate index of the walk animation
                let prev_length = pr_last - pr_first;
                let curr_length = indices.last - indices.first;
                let index = atlas.index - pr_first;
                let percentage = index as f32 / prev_length as f32;
                atlas.index = (percentage * curr_length as f32).round() as usize + indices.first;
            }
        }
        PlayerState::Jumping => {
            indices.first = JUMP_ANIMATION.0;
            indices.last = JUMP_ANIMATION.1;
            if atlas.index < indices.first || atlas.index > indices.last {
                // map to the appropriate index of the walk animation
                let prev_length = pr_last - pr_first;
                let curr_length = indices.last - indices.first;
                let index = atlas.index - pr_first;
                let percentage = index as f32 / prev_length as f32;
                atlas.index = (percentage * curr_length as f32).round() as usize + indices.first;
            }
        }
        PlayerState::Falling => {
            indices.first = FALL_ANIMATION.0;
            indices.last = FALL_ANIMATION.1;
            if atlas.index < indices.first || atlas.index > indices.last {
                // map to the appropriate index of the walk animation
                let prev_length = pr_last - pr_first;
                let curr_length = indices.last - indices.first;
                let index = atlas.index - pr_first;
                let percentage = index as f32 / prev_length as f32;
                atlas.index = (percentage * curr_length as f32).round() as usize + indices.first;
            }
        }
        _ => {}
    }
}

// system to animate the player sprite and move player entity to the right
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let (player, _) = player_query.single();
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                match player.state {
                    PlayerState::Walking | PlayerState::Running => indices.first,
                    PlayerState::Jumping | PlayerState::Falling => indices.last,
                    _ => indices.first,
                }
            } else {
                atlas.index + 1
            };
        }
    }

    // move single player entity to the right with a speed that depends on the player state
    let (player, mut transform) = player_query.single_mut();
    match player.state {
        PlayerState::Walking => {
            transform.translation.x += 1.0;
        }
        PlayerState::Running => {
            transform.translation.x += 1.5;
        }
        PlayerState::Jumping => {
            transform.translation.x += 1.0;
        }
        PlayerState::Falling => {
            transform.translation.x += 1.0;
        }
        _ => {}
    }
}

// system to continuously move the parallax layers by sending a ParallaxMoveEvent
// knowing that there is only one camera in the scene
pub fn move_camera_system(
    camera_query: Query<Entity, With<Camera>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    player_query: Query<(&Player, &Transform)>,
) {
    let (player, transform) = player_query.single();
    let camera = camera_query.get_single().unwrap();
    let mut camera_move_speed = Vec2::new(WALK_SPEED, 0.0);
    match player.state {
        PlayerState::Running => {
            camera_move_speed = Vec2::new(RUN_SPEED, 0.0);
        }
        _ => {}
    }
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed,
        camera,
    });
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let scale = Vec2::new(4.0, 4.0);
    let floor_speed = 1.0;
    let background_width = 288.0;

    // Setup your game here (camera, player, etc.)
    let camera = commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d::default(), // setup 2d camera
            ..default()
        })
        .insert(ParallaxCameraComponent::default())
        .id();

    let parallax_layers = vec![
        LayerData {
            path: FOREGROUND.to_string(),
            speed: LayerSpeed::Horizontal(0.1),
            repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
            tile_size: Vec2::new(288.0, 192.0),
            cols: 1,
            rows: 1,
            scale,
            z: 2.0,
            position: Vec2::new(0.0, scale.y * -32.0),
            ..Default::default()
        },
        LayerData {
            path: FLOOR.to_string(),
            speed: LayerSpeed::Horizontal(0.4),
            repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
            tile_size: Vec2::new(288.0, 192.0),
            cols: 1,
            rows: 1,
            scale,
            z: 1.0,
            position: Vec2::new(0.0, scale.y * -32.0),
            ..Default::default()
        },
        LayerData {
            path: MOUNTAINS.to_string(),
            speed: LayerSpeed::Horizontal(0.9),
            repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
            tile_size: Vec2::new(288.0, 192.0),
            cols: 1,
            rows: 1,
            scale,
            z: 0.4,
            position: Vec2::new(0.0, scale.y * -32.0),
            ..Default::default()
        },
        LayerData {
            path: BACKGROUND.to_string(),
            speed: LayerSpeed::Horizontal(1.0),
            repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
            tile_size: Vec2::new(288.0, 192.0),
            cols: 1,
            rows: 1,
            scale,
            z: 0.0,
            position: Vec2::new(0.0, scale.y * -32.0),
            ..Default::default()
        },
    ];

    create_parallax.send(CreateParallaxEvent {
        layers_data: parallax_layers,
        camera: camera,
    });

    // Player entity from a spritesheet
    // The spritesheet is a 4x5 grid of 16x16 sprites
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 5, 6, None, None);
    let texture = asset_server.load("player.png");
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_sprite = commands
        .spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: WALK_ANIMATION.0,
                },
                transform: Transform {
                    translation: Vec3::new(0.0, GROUND_Y, 1.5),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            },
            AnimationIndices {
                first: WALK_ANIMATION.0,
                last: FALL_ANIMATION.1,
            },
            AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
            Player {
                on_ground: true,
                state: PlayerState::Walking,
            },
        ))
        .id();
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_position: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = player_position.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        if player.on_ground {
            player.on_ground = false;
            player.state = PlayerState::Jumping;
            info!("Player state: {:?}", player.state);
            transform.translation.y += JUMP_SPEED;
        } else if player.state == PlayerState::Jumping {
            transform.translation.y += JUMP_SPEED;
            if transform.translation.y >= GROUND_Y + JUMP_HEIGHT {
                transform.translation.y = GROUND_Y + JUMP_HEIGHT;
                player.state = PlayerState::Falling;
                info!("Player state: {:?}", player.state);
            }
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 2.0; // Move left
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 2.0; // Move right
    }

    // change player state based on n key press
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        // change player state to running
        player.state = PlayerState::Running;
        info!("Player state: {:?}", player.state);
    } else if keyboard_input.just_released(KeyCode::ShiftLeft) {
        // change player state to walking
        player.state = PlayerState::Walking;
        info!("Player state: {:?}", player.state);
    }

    // if the player is on the ground, change the player state to walking
    if transform.translation.y <= GROUND_Y && !player.on_ground {
        player.on_ground = true;
        transform.translation.y = GROUND_Y;
        player.state = PlayerState::Walking;
    }
}

// apply gravity to the player entity and check if it's on the ground
fn apply_gravity(mut query: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    if !player.on_ground {
        transform.translation.y -= GRAVITY;
    }
}

fn main() {
    let scale = Vec2::new(4.0, 4.0);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platformer".to_string(),
                        resolution: (640.0, 320.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(ParallaxPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_sprite,
                move_camera_system,
                player_movement,
                apply_gravity,
                change_animation,
            ),
        )
        .run();
}
