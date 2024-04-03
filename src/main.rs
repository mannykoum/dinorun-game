use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, RepeatStrategy,
};

enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Running,
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

// system to animate the player sprite and move player entity to the right
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }

    for (_, mut transform) in &mut player_query.iter_mut() {
        transform.translation.x += 1.0;
    }
}

// system to continuously move the parallax layers by sending a ParallaxMoveEvent
// knowing that there is only one camera in the scene
pub fn move_camera_system(
    camera_query: Query<Entity, With<Camera>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    let camera = camera_query.get_single().unwrap();
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: Vec2::new(1.0, 0.0),
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
            path: "background-sunset/foreground.png".to_string(),
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
            path: "background-sunset/ground.png".to_string(),
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
            path: "background-sunset/mountains.png".to_string(),
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
            path: "background-sunset/sky.png".to_string(),
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
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 5, None, None);
    let texture = asset_server.load("player.png");
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let walk_animation_indices = AnimationIndices { first: 1, last: 12 };
    let player_sprite = commands
        .spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: walk_animation_indices.first,
                },
                transform: Transform {
                    translation: Vec3::new(0.0, scale.y * -8.0, 1.5),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            },
            walk_animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player {
                on_ground: true,
                state: PlayerState::Walking,
            },
        ))
        .id();
}

fn player_movement(
    keyboard_input: ButtonInput<KeyCode>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // Jump but only if the player is on the ground
            if transform.translation.y <= -260.0 {
                transform.translation.y += 102.0; // Move up
            }
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.0; // Move left
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.0; // Move right
        }
    }
}

// apply gravity to the player entity and check if it's on the ground
fn apply_gravity(mut query: Query<(&Player, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        if transform.translation.y > -280.0 {
            transform.translation.y -= 9.8;
        }
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
        .add_systems(Update, (animate_sprite, move_camera_system))
        .run();
}
