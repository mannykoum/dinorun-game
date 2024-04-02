use bevy::input::keyboard::*;
use bevy::prelude::*;

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
}

// Animation indices
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

// Scrolling background component
#[derive(Component)]
struct ScrollingBackground {
    speed: f32,
    width: f32,
}

// system to animate the player sprite
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
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
}

// system to scroll the background
fn scroll_background_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &ScrollingBackground), Without<Camera>>,
) {
    let mut leftmost_x = f32::INFINITY;
    let mut leftmost_entity = None;

    // Identify the leftmost background entity and its position
    for (entity, transform, _) in query.iter_mut() {
        if transform.translation.x < leftmost_x {
            leftmost_x = transform.translation.x;
            leftmost_entity = Some(entity);
        }
    }

    if let Some(entity) = leftmost_entity {
        // Now that we have the leftmost entity, we can calculate its new position outside the borrow
        if let Ok((mut transform, background, floor)) = query.get_mut(entity) {
            if transform. + background.width / 2.0 <= -400.0 {
                // This assumes there are exactly 2 backgrounds and both are always visible
                transform.translation.x += 2.0 * background.width;
            }
        }
    }

    // Apply horizontal movement to all backgrounds
    for (_, mut transform, background) in query.iter_mut() {
        transform.translation.x += background.speed * time.delta_seconds();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let scale = 4.0;
    let floor_speed = 1.0;
    let background_width = 288.0;

    // Setup your game here (camera, player, etc.)
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d::default(), // setup 2d camera
        ..default()
    });

    // Floor entity (ground) animation
    // Spawn backgrounds side by side
    let texture = asset_server.load("background-sunset/ground.png");
    for i in 0..2 {
        // Use 2 for a basic setup, increase if needed for wider views
        commands
            .spawn(SpriteBundle {
                texture: texture.clone().into(),
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * background_width * scale,
                    0.0,
                    0.0,
                )),
                ..Default::default()
            })
            .insert(ScrollingBackground {
                speed: -100.0, // Adjust the speed as needed
                width: background_width,
            })
            .insert(Floor);
    }
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.7, 0.7, 0.7),
    //             custom_size: Some(Vec2::new(288.0 * scale, 96.0 * scale)),
    //             ..default()
    //         },
    //         texture,
    //         ..default()
    //     })
    //     .insert(Floor)
    //     .insert(ScrollingBackground {
    //         speed: floor_speed,
    //         width: 288.0 * scale,
    //     });

    // Player entity from a spritesheet
    // The spritesheet is a 4x5 grid of 16x16 sprites
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 5, None, None);
    let texture = asset_server.load("player.png");
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let walk_animation_indices = AnimationIndices { first: 1, last: 12 };
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: walk_animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        walk_animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player { on_ground: true },
    ));
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
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platformer".to_string(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_sprite,
                scroll_background_system,
                player_movement,
                apply_gravity,
            ),
        )
        .run();
}
