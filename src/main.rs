use bevy::{input::keyboard::KeyboardInput, prelude::*, state::commands, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, movement)
        //.add_systems(Startup, spawn_cubes)
        //.add_plugins(HelloPlugin)
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/sonic.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.2)),
    ));
}

pub fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut transform: Query<&mut Transform, With<Sprite>>,
) {
    const VELOCITY: f32 = 500.0;
    let Ok(mut transform_obj) = transform.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let translation = &mut transform_obj.translation;

    match keys.get_pressed().next() {
        Some(KeyCode::KeyA) => {
            direction.x -= 1.0;
        }
        Some(KeyCode::KeyW) => {}
        Some(KeyCode::KeyS) => {}
        Some(KeyCode::KeyD) => {
            direction.x += 1.0;
        }
        _ => {}
    }

    *translation += direction * VELOCITY * time.delta_secs();

    // if keys.just_pressed(KeyCode::KeyA) {
    //     println!("Yahooo");
    // }

    //match eve
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

pub fn spawn_cubes(
    mut command: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn(Camera2d);
    //command.spawn(SpriteBundle {});
    command.spawn((
        Mesh2d(mesh.add(Rectangle::new(50.0, 100.0))),
        MeshMaterial2d(materials.add(Color::hsl(360. * 1 as f32 / 1 as f32, 0.95, 0.7))),
        Transform::from_xyz(100.0, 0.0, 1.0),
    ));
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, print_names);
    }
}
#[derive(Resource)]
pub struct GreetTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Jakub".to_string(),
    });
}

pub fn print_names(time: Res<Time>, mut timer: ResMut<GreetTimer>, persons_query: Query<&Person>) {
    if timer.0.tick(time.delta()).just_finished() {
        for person in persons_query.iter() {
            println!("{}", person.name);
        }
    }
}

#[derive(Component)]
pub struct Person {
    name: String,
}

pub fn hello() {
    println!("Hello");
}
