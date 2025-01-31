use light_consts::lux::{FULL_DAYLIGHT, OVERCAST_DAY};
use noise::{BasicMulti, NoiseFn, Perlin, Seedable};
use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::{css::WHITE, tailwind::*},
    input::keyboard::KeyboardInput,
    prelude::*,
    render::mesh::VertexAttributeValues,
    sprite::Anchor,
    window::PrimaryWindow,
};

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

const GIRL_PATH: &str = "models/girl.glb";

#[derive(Component, Debug)]
pub struct Movable {
    accumulation: Vec3,
    speed: f32,
    target_position: Vec3,
}

impl Movable {
    fn new(accumulation: Vec3) -> Self {
        Movable {
            accumulation,
            speed: 4.0,
            target_position: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, robot_animation)
        .add_systems(Update, move_model)
        .add_systems(Update, draw_cursor)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
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
        .spawn(DirectionalLight {
            illuminance: OVERCAST_DAY,
            shadows_enabled: true,

            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        });

    // Model - girl
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GIRL_PATH))),
        Transform::from_xyz(1.0, 0.0, 3.0),
        Movable::new(Vec3::ZERO),
    ));

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000., 1000.)
            .subdivisions(200),
    );

    let terrain_height = 20.;
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        //dbg!(positions);

        let noise = BasicMulti::<Perlin>::default();
        for pos in positions.iter_mut() {
            let val = noise.get([pos[0] as f64 / 300., pos[2] as f64 / 300.]);
            pos[1] = val as f32 * terrain_height;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, y, _]| {
                let y = *y / terrain_height;
                //dbg!(y);
                if y > 0. {
                    Color::from(GRAY_700).to_linear().to_f32_array()
                } else {
                    Color::from(GREEN_900).to_linear().to_f32_array()
                }
            })
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    terrain.compute_normals();

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        Ground,
    ));

    // Camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
    mouse_keys: Res<ButtonInput<MouseButton>>,
    mut movable: Single<&mut Movable>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };

    let point = ray.get_point(distance);

    //if let Some(target) = movable.target_position.as_mut() {

    let target = movable.target_position.as_mut();
    if mouse_keys.just_pressed(MouseButton::Right) {
        *target = point.into();
    }

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
        ),
        0.2,
        Color::WHITE,
    );
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
    mouse_keys: Res<ButtonInput<MouseButton>>,
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

        if mouse_keys.pressed(MouseButton::Right) {
            *transform = transform.looking_at(movable.target_position, Vec3::Y)
                * Transform::from_rotation(Quat::from_rotation_y(PI));
        }
        transform.translation = transform
            .translation
            .move_towards(movable.target_position, 0.1);

        // if transform.translation == facing.0 {
        //     movable.is_moving = false;
        // }

        //        transform.translation +=
        //            movable.speed * time.delta_secs() * movable.accumulation.normalize_or_zero();

        //        movable.accumulation.z = 0.;
        //        movable.accumulation.x = 0.;
    }
}

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
