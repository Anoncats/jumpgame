use bevy::{
    asset::AssetMetaCheck::Never, input::touch::Touches, prelude::*, window::WindowResolution
};
// use bevy_wind_waker_shader::prelude::*;
use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;


// Add this resource to store the camera's current position
#[derive(Resource)]
struct CameraState {
    position: Vec3,
}

#[derive(Component)]
struct Player;

fn main() {
    let asset_plugin_custom = AssetPlugin {
        meta_check: Never,
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(asset_plugin_custom)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anoncat Jump Jump".into(),
                        name: Some("anoncat jump jump.app".into()),
                        resolution: WindowResolution::new(480.,800.).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ))
        .insert_resource(CameraState {
            position: Vec3::ZERO
        })
        .add_systems(
            Startup, (
                setup_camera_and_lights,
                setup_level,
                setup_player
            ),
        )
        // .add_systems(Update, setup_animation_once_loaded.before(animate_targets))
        .add_systems(Update, (
                apply_controls.in_set(TnuaUserControlsSystemSet),
                update_camera,
            ),
        )
        .run();
}

// No Tnua-related setup here - this is just normal Bevy stuff.
fn setup_camera_and_lights(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 8.0)
            .looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0),
        ..default()
    });

    // A directly-down light to tell where the player is going to land.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::default().looking_at(-Vec3::Y, Vec3::Z),
        ..Default::default()
    });
}

fn update_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
    mut camera_state: ResMut<CameraState>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    // Adjust these values to bring the camera closer
    let camera_offset = Vec3::new(0.0, 2.5, 8.0);
    let look_at_offset = Vec3::new(0.0, 1.5, 0.0);

    let target = player_transform.translation + camera_offset;

    // Smoothly interpolate the camera position
    let smoothness = 5.0; // Adjust this value to change the smoothing amount (higher = smoother)
    camera_state.position = camera_state.position.lerp(target,
        smoothness * time.delta_seconds(),
    );

    camera_transform.translation = camera_state.position;
    camera_transform.look_at(player_transform.translation + look_at_offset, Vec3::Y);
}


// No Tnua-related setup here - this is just normal Bevy (and Avian) stuff.
fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let floor_handle = asset_server.load("floor.glb#Scene0");

    // Spawn floor tiles and scale them
    let size = 20.0;

    commands.spawn((
        SceneBundle {
            scene: floor_handle.clone(),
            transform: Transform::from_scale(Vec3::new(
                size,
                1.0,
                size
            )).with_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::cuboid(
            size,
            1.0,
            size,
        ),
    ));
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    // mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // build animation graph
    // let mut graph = AnimationGraph::new();
    // let animations_arr: Vec<Handle<AnimationClip>> = (0..8).map(|i| {
    //     asset_server.load(&format!("cat4.glb#Animation{}", i))
    // }).collect();
    // let animations= graph.add_clips(animations_arr, 1.0, graph.root).collect();

    // let graph = graphs.add(graph);

    // commands.insert_resource(Animations {
    //     animations,
    //     graph: graph.clone(),
    // });

    // Spawn the player
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("cat4-resample-no-animation.glb")),
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.4, 0.6),
        TnuaControllerBundle::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.4, 0.6)),
        LockedAxes::ROTATION_LOCKED,
        Player
    ));

}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    _touches: Res<Touches>,
    mut query: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    // Keyboard controls

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Touch controls
    // if let Some(touch) = touches.first() {
    //     let window = get_primary_window_size(); // You'll need to implement this function
    //     let touch_position = touch.position();

    //     // Divide the screen into four quadrants for directional control
    //     if touch_position.x < window.width / 2.0 {
    //         direction -= Vec3::X; // Left
    //     } else {
    //         direction += Vec3::X; // Right
    //     }
    //     if touch_position.y < window.height / 2.0 {
    //         direction += Vec3::Z; // Down
    //     } else {
    //         direction -= Vec3::Z; // Up
    //     }
    // }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 0.4,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });

    }
}
