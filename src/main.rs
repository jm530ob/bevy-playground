use std::time::Duration;

use bevy::{
    color::palettes::css::{DARK_GREEN, RED},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const GIRL_PATH: &str = "models/girl.glb";

#[derive(Component)]
pub struct Movable {
    accumulation: Vec3,
    speed: f32,
}

impl Movable {
    fn new(accumulation: Vec3) -> Self {
        Movable {
            accumulation,
            speed: 4.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, robot_animation)
        .add_systems(Update, move_model)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text::new("Fps: "),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GIRL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GIRL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(GIRL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(3).from_asset(GIRL_PATH)),
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });

    commands
        .spawn(PointLight {
            shadows_enabled: true,
            ..default()
        })
        .insert(Transform::from_xyz(0.0, 10.0, 0.0));

    // Model - girl
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GIRL_PATH))),
        Transform::from_xyz(1.0, 0.0, 3.0),
        Movable::new(Vec3::ZERO),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::from(DARK_GREEN))),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

pub fn robot_animation(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[1], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

pub fn move_model(
    mut model_q: Query<(&mut Transform, &mut Movable)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut movable) in model_q.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            movable.accumulation.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            movable.accumulation.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            movable.accumulation.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            movable.accumulation.x += 1.0;
        }

        transform.translation +=
            movable.speed * time.delta_secs() * movable.accumulation.normalize_or_zero();

        movable.accumulation.z = 0.;
        movable.accumulation.x = 0.;
    }
}

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
