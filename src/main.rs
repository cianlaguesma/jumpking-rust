use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct Player {
    velocity: Vec2,
    horizontal_acceleration: f32,
}

struct Wall{
    position: Vec2,
    width: f32,
    height: f32,
}
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, keyboard_input_system)
    .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}


const X_EXTENT: f32 = 600.;
const X_DECAY: f32 = 1.;
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let jump_king_sprite = asset_server.load("jumpking_sprite.png");
    asset_server.load_state(&jump_king_sprite);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: jump_king_sprite,
            transform: Transform::from_xyz(300.,0.,0.),
            ..default()
        },
        Player{
            velocity: Vec2 {x:0., y:0.},
            horizontal_acceleration: 1.,
        },
    ));
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut sprite_position: Query<(&mut Transform, &mut Player, &Velocity, &Acceleration)>, time: Res<Time>){
    for (mut transform, mut player, velocity, acceleration) in &mut sprite_position {
        // Implement jump frames when space is pressed
        if keyboard_input.pressed(KeyCode::KeyW){
            player.velocity.y += velocity.y * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            player.velocity.y = velocity.y * time.delta_seconds() + -0.5 * acceleration.0 * (time.delta_seconds()).powf(2.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            
            player.velocity.x -= player.horizontal_acceleration * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            
            player.velocity.x += player.horizontal_acceleration * time.delta_seconds();
        }

        transform.translation.x += player.velocity.x;
    }
    
}