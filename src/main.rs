use std::thread::current;

use bevy::prelude::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
const BACKGROUND_COLOR: Color = Color::rgb(1., 1., 1.);
const SPEED: f32 = 50.0;
const PLAYER_SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 500.0;
const MAX_JUMP_FRAMES : f32 = 5.;
const WINDOW: Vec2 = Vec2::new(800., 600.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            FixedUpdate,
            (
                gravity_system,
                collision_system,
                movement_system,
            )
                .chain(),
        )
        .add_systems(Update, (jump_system,move_player).chain())
        .run();
}

#[derive(Component)]
struct Walls{
    size: Vec2,
    position: Vec3,
}
#[derive(Component)]
struct Gravity(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct CanJump(bool);

#[derive(Component)]

struct JumpFrames(f32);

#[derive(Component)]
struct OnGround(bool);

#[derive(Component)]
struct Wall;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let jump_king_sprite = asset_server.load("jumpking_sprite.png");
    asset_server.load_state(&jump_king_sprite);

    let platforms = [
        Walls {
            size: Vec2 { x:10000.0, y:50.},
            position: Vec3 {x:0., y: -(WINDOW.y)+250., z:0.},
        },
        Walls {
            size: Vec2 {x:50., y:10000.0},
            position: Vec3 {x:-(WINDOW.x)+150., y:0., z:0.},
        },
        
        Walls {
            size: Vec2 {x:50., y:10000.0},
            position: Vec3 {x:(WINDOW.x)-150., y:0., z:0.},
        }
    ];

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: jump_king_sprite,
            transform: Transform {
                translation: Vec3::new(5., 5., 0.),
                scale: Vec3::new(1.,1.,1.),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        },
        Velocity(Vec2::new(0., 0.)),
        Gravity(19.6),
        CanJump(true),
        OnGround(false),
        JumpFrames(0.),
        Player,
    ));
    for (idx, wall) in platforms.into_iter().enumerate(){
        commands.spawn((
            Wall,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 0.2,
                        green: 0.3,
                        blue: 0.4,
                        alpha: 1.,
                    },
                    custom_size: Some(Vec2::new(wall.size.x, wall.size.y)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(wall.position.x, wall.position.y, wall.position.z),
                    scale: Vec3::new(1.,1.,1.),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &CanJump, &mut JumpFrames), With<Player>>,
    time: Res<Time>,
) {
    let (mut velocity, can_jump, mut current_jump_frames) = player_query.single_mut();
    let mut x_axis = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        x_axis -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        x_axis += 1.0;
    }

    velocity.0.x = x_axis * PLAYER_SPEED;

    if keyboard_input.just_released(KeyCode::Space) && can_jump.0 {
        println!("released");
        println!("{}", current_jump_frames.0);
        while current_jump_frames.0 > 0. {
            current_jump_frames.0 -= 2.;
            velocity.0.y += JUMP_FORCE * time.delta_seconds();  
        }
        if current_jump_frames.0 < 0.{
            current_jump_frames.0 = 0.;
        }
    }

    if keyboard_input.pressed(KeyCode::Space) && can_jump.0{
        if current_jump_frames.0 < MAX_JUMP_FRAMES{
            println!("{}", current_jump_frames.0);
            current_jump_frames.0 += 0.1;
        }
    }

}

fn movement_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let (mut transform, velocity) = query.single_mut();
    transform.translation.x += velocity.0.x * SPEED * time.delta_seconds();
    transform.translation.y += velocity.0.y * SPEED * time.delta_seconds();
}

fn gravity_system(mut query: Query<(&Gravity, &mut Velocity)>, time: Res<Time>) {
    let (gravity, mut velocity) = query.single_mut();
    velocity.0.y -= gravity.0 * time.delta_seconds();
}

fn jump_system(mut query: Query<(&Velocity, &mut CanJump), With<Player>>){
let (velocity, mut can_jump) = query.single_mut();

    //Player can only jump if he is on the ground, so if he jumped then he cannot jump anymore until he collides with the ground
    if velocity.0.y > 0. {
        can_jump.0 = false;
    }
}

fn collision_system(
    mut player_query: Query<(&mut Transform, &mut Velocity, &Sprite, &mut CanJump), Without<Wall>>,
    wall_query: Query<(&Transform, &Sprite), With<Wall>>,
) {
    let (mut player_position, mut player_velocity, player_sprite, mut can_jump) = player_query.single_mut();
    let player_size = player_sprite.custom_size.unwrap();

    for (wall_position, wall_sprite) in wall_query.iter() {
        let wall_size = wall_sprite.custom_size.unwrap();

        //Utilize AABB of Bevy, inputs the center and half_size
        let walls = Aabb2d::new(wall_position.translation.truncate(), wall_size /2.);
        let player = Aabb2d::new(player_position.translation.truncate(), player_size /2.);

        let mut player_dims_y = player_position.translation.y + player_size.y / 2.0;
        let mut wall_dims_y = wall_position.translation.y + wall_size.y / 2.0;

        let mut player_dims_x = player_position.translation.x + player_size.x / 2.0;
        let mut wall_dims_x = wall_position.translation.x + wall_size.x / 2.0;

        if player.intersects(&walls){
            //Check if wall is a platform
            if wall_size.x < wall_size.y{
                //Collision is on the right
                if player_velocity.0.x > 0.{
                    if player_dims_x < wall_dims_x
                    {
                        println!("Getting hit on the right!");

                        // Bounces the player back to the correct position
                        player_dims_x += player_size.x/2.;
                        wall_dims_x -= wall_size.x/2.;

                        // 3rd value is gap
                        player_position.translation.x += wall_dims_x - player_dims_x - 0.;

                        // Collision detected, stop the player's horizontal movement
                        player_velocity.0.x = player_velocity.0.x.max(0.0);
                    }
                }

                //Collision is on the left
                if player_velocity.0.x < 0.{
                    if player_dims_x > wall_dims_x
                    {
                        println!("Getting hit on the left!");
                        
                        player_dims_x -= player_size.x/2.;
                        wall_dims_x += wall_size.x/2.;
                        player_position.translation.x += wall_dims_x - player_dims_x - 0.;

                        // Collision detected, stop the player's horizontal movement
                        player_velocity.0.x = player_velocity.0.x.max(0.0);
                    }
                }
            //Not a platform, but a wall
            }else{
                if player_velocity.0.y <= 0. {
                    if player_dims_y > wall_dims_y
                    {
                        // println!("Getting hit below!");
                        // He now can jump because he collides with the ground
                        can_jump.0 = true;
                        player_dims_y -= player_size.y/2.;
                        wall_dims_y += wall_size.y/2.;
                        player_position.translation.y += wall_dims_y - player_dims_y - 0.;

                        // Collision detected, stop the player's vertical movement
                        player_velocity.0.y = player_velocity.0.y.max(0.0);
                    }
                }

                if player_velocity.0.y > 0.{
                    if player_dims_y < wall_dims_y
                    {
                        println!("Getting hit above!");

                        player_dims_y += player_size.y/2.;
                        wall_dims_y -= wall_size.y/2.;
                        player_position.translation.y += wall_dims_y - player_dims_y + 1.;

                        // Collision detected, stop the player's vertical movement
                        player_velocity.0.y = player_velocity.0.y.max(0.0);
                    }
                }
            }
        }
    }
}
