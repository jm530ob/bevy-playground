use std::time::Duration;

use bevy::{
    color::palettes::css::DARK_GREEN,
    input::keyboard::KeyboardInput,
    prelude::*,
    state::commands,
    window::{PrimaryWindow, WindowMode},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const GIRL_PATH: &str = "models/girl.glb";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, robot_animation)
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
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GIRL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GIRL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(GIRL_PATH)),
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });

    //commands.insert_resource(Animations(vec![
    //    asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/robot.glb"))
    //]));
    // window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GIRL_PATH))),
        Transform::from_xyz(1.0, 1.0, 3.0),
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
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
