use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(1., 1., 1.);
const SPEED: f32 = 50.0;
const PLAYER_SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 500.0;

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
                move_player,
            )
                .chain(),
        )
        .run();
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
struct Ground;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let jump_king_sprite = asset_server.load("jumpking_sprite.png");
    asset_server.load_state(&jump_king_sprite);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: jump_king_sprite,
            transform: Transform {
                translation: Vec3::new(5., 5., 0.),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        },
        Velocity(Vec2::new(0., 0.)),
        Gravity(9.8),
        CanJump(true),
        Player,
    ));

    commands.spawn((
        Ground,
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.2,
                    green: 0.3,
                    blue: 0.4,
                    alpha: 1.,
                },
                custom_size: Some(Vec2::new(10000., 10.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -350., 0.),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &CanJump), With<Player>>,
    time: Res<Time>,
) {
    let (mut velocity, can_jump) = player_query.single_mut();
    let mut x_axis = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        x_axis -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        x_axis += 1.0;
    }

    velocity.0.x = x_axis * PLAYER_SPEED;

    if keyboard_input.just_pressed(KeyCode::Space) && can_jump.0 {
        velocity.0.y = JUMP_FORCE * time.delta_seconds();
    }
}

fn movement_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * SPEED * time.delta_seconds();
        transform.translation.y += velocity.0.y * SPEED * time.delta_seconds();
    }
}

fn gravity_system(mut query: Query<(&Gravity, &mut Velocity)>, time: Res<Time>) {
    for (gravity, mut velocity) in query.iter_mut() {
        velocity.0.y -= gravity.0 * time.delta_seconds();
    }
}

fn collision_system(
    mut player_query: Query<(&Transform, &mut Velocity, &Sprite), With<Player>>,
    ground_query: Query<(&Transform, &Sprite), With<Ground>>,
) {
    let (player_transform, mut player_velocity, player_sprite) = player_query.single_mut();
    let player_size = player_sprite.custom_size.unwrap();

    for (ground_transform, ground_sprite) in ground_query.iter() {
        let ground_size = ground_sprite.custom_size.unwrap();

        // Perform AABB collision detection
        if player_transform.translation.y - player_size.y / 2.0 + 2.0
            < ground_transform.translation.y + ground_size.y / 2.0
        {
            // Collision detected, stop the player's downward movement
            player_velocity.0.y = player_velocity.0.y.max(0.0);
        }
    }
}
