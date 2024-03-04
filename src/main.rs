use bevy::prelude::*;

// Plugin from bevy tutorial
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (greet_people, update_people).chain()));
    }
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Manos Koumantakis".to_string())));
    commands.spawn((Person, Name("Myrto Karampesini-Matska".to_string())));
    commands.spawn((Person, Name("Thodoris Chalimas".to_string())));
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Manos Koumantakis" {
            name.0 = "Manos Koumandakis".to_string();
            break;
        }
    }
}

// Floor component
#[derive(Component)]
struct Floor;

// Player component
#[derive(Component)]
struct Player {
    on_ground: bool,
}

fn setup(mut commands: Commands) {
    // Setup your game here (camera, player, etc.)
    commands.spawn(Camera2dBundle::default());

    // Floor entity
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(800.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .insert(Floor);

    // Player entity
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(30.0, 60.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { on_ground: false });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            // Jump but only if the player is on the ground
            if transform.translation.y <= -260.0 {
                transform.translation.y += 102.0; // Move up
            }
        } else if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0; // Move left
        } else if keyboard_input.pressed(KeyCode::Right) {
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
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
