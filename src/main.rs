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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup your game here (camera, player, etc.)
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d::default(), // setup 2d camera
        ..default()
    });

    // Floor entity (ground) animation
    let texture = asset_server.load("background-sunset/ground.png");
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(640.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        })
        .insert(Floor);

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
        .add_systems(Update, (animate_sprite))
        .run();
}
