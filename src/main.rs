use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        Direction::Up,
    ));
    let shapes=[
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
    ];

    let num_shapes = shapes.len();

    for(i, shape) in shapes.into_iter().enumerate(){
        let color = Color::hsl(360. * i as f32/num_shapes as f32, 0.95, 0.7);

        commands.spawn((MaterialMesh2dBundle{
            mesh:shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                -X_EXTENT /2. + i as f32 / (num_shapes -1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        },
        Direction::Up,
        ));
    }
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut sprite_position: Query<(&mut Direction, &mut Transform)>, time: Res<Time>){
    for (mut logo, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::KeyW){
            transform.translation.y += 150. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS){
    
            transform.translation.y -= 150. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            
            transform.translation.x -= 150. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
    
}